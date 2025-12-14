mod mem_node;
mod memory;
mod node_set;

use mem_node::MemNode;
use memory::{NumaMemory, PlacementMode};
use node_set::NodeSet;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
// 内存分配模式枚举
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum NumaMode {
    /// 交错模式：内存页面在多个节点间交错分配
    Interleave,
    /// 严格模式：内存必须从指定节点分配
    #[default]
    Strict,
    /// 首选模式：优先从指定节点分配，但不强制
    Preferred,
    /// 限制模式：使用系统默认策略，仅用cgroups限制节点
    Restrictive,
}
// 主结构体：NUMA 调优配置
#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct NumaTune {
    #[serde(rename = "memory", skip_serializing_if = "Option::is_none")]
    pub memory: Option<NumaMemory>,

    #[serde(rename = "memnode", skip_serializing_if = "Option::is_none")]
    pub memnodes: Option<Vec<MemNode>>,
}
impl NumaTune {
    /// 创建新的 NUMA 配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置全局内存配置
    pub fn with_memory(mut self, memory: NumaMemory) -> Self {
        self.memory = Some(memory);
        self
    }

    /// 添加单个节点配置
    pub fn add_memnode(mut self, memnode: MemNode) -> Self {
        if self.memnodes.is_none() {
            self.memnodes = Some(Vec::new());
        }

        if let Some(ref mut nodes) = self.memnodes {
            // 确保没有重复的 cell_id
            if !nodes.iter().any(|n| n.cell_id == memnode.cell_id) {
                nodes.push(memnode);
            }
        }

        self
    }

    /// 批量添加节点配置
    pub fn with_memnodes(mut self, memnodes: Vec<MemNode>) -> Self {
        self.memnodes = Some(memnodes);
        self
    }

    /// 获取指定 cell_id 的节点配置
    pub fn get_memnode(&self, cell_id: u32) -> Option<&MemNode> {
        self.memnodes
            .as_ref()
            .and_then(|nodes| nodes.iter().find(|n| n.cell_id == cell_id))
    }

    /// 获取所有 cell_id
    pub fn get_cell_ids(&self) -> Vec<u32> {
        self.memnodes
            .as_ref()
            .map(|nodes| nodes.iter().map(|n| n.cell_id).collect())
            .unwrap_or_default()
    }

    /// 验证配置的有效性
    pub fn validate(&self, total_cells: u32, host_nodes: u32) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // 验证全局内存配置
        if let Some(memory) = &self.memory {
            // 如果 placement 是 auto，则 nodeset 应被忽略
            if let Some(PlacementMode::Auto) = memory.placement {
                if memory.nodeset.is_some() {
                    errors.push("当 placement='auto' 时，nodeset 将被忽略".to_string());
                }
            } else {
                // 验证 nodeset
                if let Some(nodeset) = &memory.nodeset {
                    match nodeset.validate(Some(host_nodes)) {
                        Ok(_) => {}
                        Err(mut ns_errors) => {
                            errors.append(&mut ns_errors);
                        }
                    }
                }
            }

            // 验证模式
            if let Some(mode) = &memory.mode
                && *mode == NumaMode::Restrictive
                && self.memnodes.is_some()
            {
                // 检查 memnode 是否也使用 restrictive 模式
                if let Some(nodes) = &self.memnodes {
                    for node in nodes {
                        if node.mode != NumaMode::Restrictive {
                            errors.push(format!(
                                    "当 memory 模式为 'restrictive' 时，memnode cellid={} 也必须使用 'restrictive' 模式",
                                    node.cell_id
                                ));
                        }
                    }
                }
            }
        }

        // 验证 memnode 配置
        if let Some(nodes) = &self.memnodes {
            // 检查是否有重复的 cell_id
            let mut seen_cells = HashSet::new();
            for node in nodes {
                if !seen_cells.insert(node.cell_id) {
                    errors.push(format!("重复的 cell_id: {}", node.cell_id));
                }

                // 验证 cell_id 范围
                if node.cell_id >= total_cells {
                    errors.push(format!(
                        "cell_id {} 超出最大 guest NUMA 节点数 {}",
                        node.cell_id,
                        total_cells - 1
                    ));
                }

                // 验证节点集
                match node.nodeset.validate(Some(host_nodes)) {
                    Ok(_) => {}
                    Err(mut ns_errors) => {
                        errors.append(&mut ns_errors);
                    }
                }
            }

            // 检查是否与自动放置兼容
            if let Some(memory) = &self.memory
                && let Some(PlacementMode::Auto) = memory.placement
            {
                errors.push("memnode 配置与自动放置模式不兼容".to_string());
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// 生成摘要信息
    pub fn get_summary(&self) -> String {
        let mut summary = String::from("NUMA Tuning Configuration:\n");

        if let Some(memory) = &self.memory {
            summary.push_str("Global Memory:\n");
            if let Some(mode) = &memory.mode {
                summary.push_str(&format!("  Mode: {:?}\n", mode));
            }
            if let Some(nodeset) = &memory.nodeset {
                summary.push_str(&format!("  Nodeset: {}\n", nodeset.expression));
            }
            if let Some(placement) = &memory.placement {
                summary.push_str(&format!("  Placement: {:?}\n", placement));
            }
        }

        if let Some(nodes) = &self.memnodes {
            summary.push_str("Per-node Configuration:\n");
            for node in nodes {
                summary.push_str(&format!(
                    "  Cell {}: mode={:?}, nodeset={}\n",
                    node.cell_id, node.mode, node.nodeset.expression
                ));
            }
        }

        summary
    }

    /// 计算内存绑定策略
    pub fn calculate_memory_binding(&self) -> HashMap<u32, HashSet<u32>> {
        let mut binding = HashMap::new();

        // 处理 memnode 配置
        if let Some(nodes) = &self.memnodes {
            for node in nodes {
                if let Ok(host_nodes) = node.nodeset.parse() {
                    binding.insert(node.cell_id, host_nodes);
                }
            }
        }

        // 为未配置的 cell_id 使用默认配置
        if let Some(memory) = &self.memory
            && let Some(nodeset) = &memory.nodeset
            && let Ok(default_nodes) = nodeset.parse()
        {
            // 获取所有已配置的 cell_id
            let configured_cells: HashSet<u32> = binding.keys().copied().collect();

            // 为未配置的 cell_id 添加默认绑定
            if let Some(max_cell) = configured_cells.iter().max() {
                for cell_id in 0..=*max_cell {
                    if !configured_cells.contains(&cell_id) {
                        binding.insert(cell_id, default_nodes.clone());
                    }
                }
            }
        }

        binding
    }
}
