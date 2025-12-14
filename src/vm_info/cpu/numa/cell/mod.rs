mod cache;
mod distances;

use crate::MemoryUnit;
use crate::vm_info::cpu::numa::interconnects::InterconnectConfig;
pub use cache::*;
pub use distances::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// NUMA 节点丢弃策略
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum DiscardPolicy {
    Yes,
    No,
}

/// NUMA 节点内存访问模式
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum MemoryAccess {
    Shared,
    Private,
}

/// NUMA 节点配置
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NumaCell {
    /// 节点ID
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,

    /// CPU集合
    #[serde(rename = "@cpus", skip_serializing_if = "Option::is_none")]
    pub cpus: Option<String>,

    /// 内存大小
    #[serde(rename = "@memory")]
    pub memory: u64,

    /// 内存单位
    #[serde(rename = "@unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<MemoryUnit>,

    /// 内存访问模式
    #[serde(rename = "@memAccess", skip_serializing_if = "Option::is_none")]
    pub mem_access: Option<MemoryAccess>,

    /// 丢弃策略
    #[serde(rename = "@discard", skip_serializing_if = "Option::is_none")]
    pub discard: Option<DiscardPolicy>,

    /// 缓存配置（可重复）
    #[serde(rename = "cache", skip_serializing_if = "Option::is_none")]
    pub caches: Option<Vec<CacheConfig>>,

    /// 距离表
    #[serde(rename = "distances", skip_serializing_if = "Option::is_none")]
    pub distances: Option<DistanceTable>,

    /// 互连配置
    #[serde(rename = "interconnects", skip_serializing_if = "Option::is_none")]
    pub interconnects: Option<InterconnectConfig>,
}

impl NumaCell {
    pub fn new(memory: u64) -> Self {
        Self {
            id: None,
            cpus: None,
            memory,
            unit: None,
            mem_access: None,
            discard: None,
            caches: None,
            distances: None,
            interconnects: None,
        }
    }

    pub fn with_id(mut self, id: u32) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_cpus(mut self, cpus: &str) -> Self {
        self.cpus = Some(cpus.to_string());
        self
    }

    pub fn with_unit(mut self, unit: MemoryUnit) -> Self {
        self.unit = Some(unit);
        self
    }

    pub fn with_mem_access(mut self, mem_access: MemoryAccess) -> Self {
        self.mem_access = Some(mem_access);
        self
    }

    pub fn with_discard(mut self, discard: DiscardPolicy) -> Self {
        self.discard = Some(discard);
        self
    }

    pub fn add_cache(mut self, cache: CacheConfig) -> Self {
        if self.caches.is_none() {
            self.caches = Some(Vec::new());
        }

        if let Some(ref mut caches) = self.caches {
            caches.push(cache);
        }

        self
    }

    pub fn with_distances(mut self, distances: DistanceTable) -> Self {
        self.distances = Some(distances);
        self
    }

    pub fn with_interconnects(mut self, interconnects: InterconnectConfig) -> Self {
        self.interconnects = Some(interconnects);
        self
    }

    /// 解析CPU集合
    pub fn parse_cpus(&self) -> Result<HashSet<u32>, String> {
        let mut cpus = HashSet::new();

        if let Some(cpus_str) = &self.cpus {
            let parts: Vec<&str> = cpus_str.split(',').collect();

            for part in parts {
                let part = part.trim();
                if part.is_empty() {
                    continue;
                }

                // 处理范围
                if part.contains('-') {
                    let range_parts: Vec<&str> = part.split('-').collect();
                    if range_parts.len() != 2 {
                        return Err(format!("Invalid CPU range: {}", part));
                    }

                    let start = range_parts[0]
                        .parse::<u32>()
                        .map_err(|_| format!("Invalid start CPU: {}", range_parts[0]))?;
                    let end = range_parts[1]
                        .parse::<u32>()
                        .map_err(|_| format!("Invalid end CPU: {}", range_parts[1]))?;

                    if start > end {
                        return Err(format!("Invalid range: start ({}) > end ({})", start, end));
                    }

                    for cpu in start..=end {
                        cpus.insert(cpu);
                    }
                } else {
                    // 单个CPU
                    let cpu = part
                        .parse::<u32>()
                        .map_err(|_| format!("Invalid CPU: {}", part))?;
                    cpus.insert(cpu);
                }
            }
        }

        Ok(cpus)
    }

    /// 获取CPU数量
    pub fn get_cpu_count(&self) -> Result<usize, String> {
        self.parse_cpus().map(|cpus| cpus.len())
    }

    /// 获取内存字节数
    pub fn get_memory_bytes(&self) -> u64 {
        self.unit.unwrap_or_default().to_bytes(self.memory)
    }

    /// 验证节点配置
    pub fn validate(&self, cell_index: u32, total_cells: u32) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // 检查ID一致性
        if let Some(id) = self.id
            && id != cell_index
        {
            errors.push(format!(
                "Cell id mismatch: expected {}, got {}",
                cell_index, id
            ));
        }

        // 验证CPU集合
        if let Some(cpus_str) = &self.cpus {
            if cpus_str.is_empty() {
                errors.push("CPU set cannot be empty string".to_string());
            } else {
                match self.parse_cpus() {
                    Ok(cpus) => {
                        if cpus.is_empty() {
                            errors.push("CPU set is empty after parsing".to_string());
                        }

                        // 检查CPU编号的合理性
                        for &cpu in &cpus {
                            if cpu > 1023 {
                                // 合理上限
                                errors.push(format!("CPU number {} is unusually high", cpu));
                            }
                        }
                    }
                    Err(e) => {
                        errors.push(format!("Failed to parse CPU set '{}': {}", cpus_str, e));
                    }
                }
            }
        }

        // 验证内存大小
        if self.memory == 0 {
            errors.push("Memory size cannot be 0".to_string());
        }

        let memory_bytes = self.get_memory_bytes();
        if memory_bytes < 64 * 1024 {
            // 64 KiB
            errors.push(format!(
                "Memory size {} bytes is too small for a NUMA node",
                memory_bytes
            ));
        }

        // 验证缓存配置
        if let Some(caches) = &self.caches {
            for (index, cache) in caches.iter().enumerate() {
                match cache.validate() {
                    Ok(_) => {}
                    Err(mut cache_errors) => {
                        errors.push(format!("Cache {} (index {}):", cache.level, index));
                        errors.append(&mut cache_errors);
                    }
                }
            }

            // 检查缓存级别重复
            let mut seen_levels = HashSet::new();
            for cache in caches {
                if !seen_levels.insert(cache.level) {
                    errors.push(format!(
                        "Duplicate cache level {} in NUMA cell {}",
                        cache.level, cell_index
                    ));
                }
            }
        }

        // 验证距离表
        if let Some(distances) = &self.distances {
            match distances.validate(cell_index, total_cells) {
                Ok(_) => {}
                Err(mut distance_errors) => {
                    errors.append(&mut distance_errors);
                }
            }
        }

        // 验证互连配置
        if let Some(interconnects) = &self.interconnects {
            match interconnects.validate(total_cells) {
                Ok(_) => {}
                Err(mut interconnect_errors) => {
                    errors.append(&mut interconnect_errors);
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
