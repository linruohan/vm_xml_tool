mod cell;
mod interconnects;

use crate::MemoryUnit;
use cell::NumaCell;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// NUMA 拓扑配置
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NumaTopology {
    #[serde(rename = "cell")]
    pub cells: Vec<NumaCell>,
}

impl NumaTopology {
    pub fn new() -> Self {
        Self { cells: Vec::new() }
    }

    pub fn add_cell(mut self, cell: NumaCell) -> Self {
        self.cells.push(cell);
        self
    }

    pub fn with_cells(mut self, cells: Vec<NumaCell>) -> Self {
        self.cells = cells;
        self
    }

    /// 验证整个NUMA拓扑
    pub fn validate(&self, total_vcpus: u32) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        let total_cells = self.cells.len() as u32;

        if self.cells.is_empty() {
            errors.push("NUMA topology must have at least one cell".to_string());
            return Err(errors);
        }

        // 验证每个节点
        for (index, cell) in self.cells.iter().enumerate() {
            match cell.validate(index as u32, total_cells) {
                Ok(_) => {}
                Err(mut cell_errors) => {
                    errors.push(format!("NUMA cell {}:", index));
                    errors.append(&mut cell_errors);
                }
            }
        }

        // 检查CPU分配
        let cpu_validation = self.validate_cpu_allocation(total_vcpus);
        if let Err(mut cpu_errors) = cpu_validation {
            errors.append(&mut cpu_errors);
        }

        // 检查ID一致性
        let id_validation = self.validate_cell_ids();
        if let Err(mut id_errors) = id_validation {
            errors.append(&mut id_errors);
        }

        // 检查内存分配合理性
        let memory_validation = self.validate_memory_allocation();
        if let Err(mut memory_errors) = memory_validation {
            errors.append(&mut memory_errors);
        }

        // 检查距离表一致性
        let distance_validation = self.validate_distance_tables();
        if let Err(mut distance_errors) = distance_validation {
            errors.append(&mut distance_errors);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// 验证CPU分配
    fn validate_cpu_allocation(&self, total_vcpus: u32) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        let mut all_cpus = HashSet::new();
        let mut total_allocated_cpus = 0;

        for (cell_index, cell) in self.cells.iter().enumerate() {
            match cell.parse_cpus() {
                Ok(cpus) => {
                    total_allocated_cpus += cpus.len();

                    // 检查CPU是否重复分配
                    for cpu in &cpus {
                        if !all_cpus.insert(*cpu) {
                            errors.push(format!("CPU {} is assigned to multiple NUMA cells", cpu));
                        }

                        // 检查CPU编号是否有效
                        if *cpu >= total_vcpus {
                            errors.push(format!(
                                "CPU {} exceeds total vCPUs ({}) in cell {}",
                                cpu, total_vcpus, cell_index
                            ));
                        }
                    }
                }
                Err(e) => {
                    errors.push(format!("Cell {}: {}", cell_index, e));
                }
            }
        }

        // 检查CPU总数
        if total_allocated_cpus > total_vcpus as usize {
            errors.push(format!(
                "Total allocated CPUs ({}) exceeds total vCPUs ({})",
                total_allocated_cpus, total_vcpus
            ));
        }

        // 检查是否所有CPU都被分配
        if total_allocated_cpus < total_vcpus as usize {
            errors.push(format!(
                "Not all vCPUs are assigned to NUMA cells: {} allocated, {} total",
                total_allocated_cpus, total_vcpus
            ));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// 验证节点ID
    fn validate_cell_ids(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        let mut seen_ids = HashSet::new();

        // 检查是否有混合使用ID的情况
        let has_explicit_ids = self.cells.iter().any(|c| c.id.is_some());
        let has_implicit_ids = self.cells.iter().any(|c| c.id.is_none());

        if has_explicit_ids && has_implicit_ids {
            errors
                .push("Mixing cells with and without id attribute is not recommended".to_string());
        }

        // 检查重复的ID
        for (index, cell) in self.cells.iter().enumerate() {
            if let Some(id) = cell.id {
                if !seen_ids.insert(id) {
                    errors.push(format!("Duplicate cell id: {}", id));
                }

                // 检查ID是否连续
                if id != index as u32 {
                    errors.push(format!(
                        "Cell id {} at position {} breaks sequence",
                        id, index
                    ));
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// 验证内存分配
    fn validate_memory_allocation(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        let mut total_memory = 0u64;

        for (index, cell) in self.cells.iter().enumerate() {
            let memory_bytes = cell.get_memory_bytes();
            total_memory += memory_bytes;

            // 检查内存大小是否合理
            if memory_bytes % 4096 != 0 {
                errors.push(format!(
                    "Memory size in cell {} is not page-aligned ({} bytes)",
                    index, memory_bytes
                ));
            }

            // 检查是否过小
            if memory_bytes < 64 * 1024 * 1024 {
                // 64 MiB
                errors.push(format!(
                    "Memory in cell {} is very small ({} MiB)",
                    index,
                    memory_bytes / 1024 / 1024
                ));
            }
        }

        // 检查总内存
        if total_memory == 0 {
            errors.push("Total NUMA memory cannot be 0".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// 验证距离表一致性
    fn validate_distance_tables(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        let total_cells = self.cells.len();

        // 收集所有距离表
        let mut distance_matrices = Vec::new();

        for (i, cell) in self.cells.iter().enumerate() {
            if let Some(distances) = &cell.distances {
                let mut matrix = vec![0u32; total_cells];

                for sibling in &distances.siblings {
                    if sibling.cell_id < total_cells as u32 {
                        matrix[sibling.cell_id as usize] = sibling.value;
                    }
                }

                distance_matrices.push(matrix);
            } else {
                // 如果没有距离表，创建默认的
                let mut matrix = vec![20u32; total_cells];
                matrix[i] = 10; // 本地距离
                distance_matrices.push(matrix);
            }
        }

        // 检查距离表是否对称
        for (i, row_i) in distance_matrices.iter().enumerate() {
            for (j, row_j) in distance_matrices.iter().enumerate().skip(i + 1) {
                let distance_ij = row_i[j];
                let distance_ji = row_j[i];

                if distance_ij != distance_ji {
                    errors.push(format!(
                        "Distance matrix is not symmetric: cell{}->cell{} = {}, but cell{}->cell{} = {}",
                        i, j, distance_ij, j, i, distance_ji
                    ));
                }
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// 获取总内存
    pub fn get_total_memory(&self) -> u64 {
        self.cells.iter().map(|cell| cell.get_memory_bytes()).sum()
    }

    /// 获取总CPU数
    pub fn get_total_cpus(&self) -> Result<usize, String> {
        let mut total = 0;

        for cell in &self.cells {
            total += cell.get_cpu_count()?;
        }

        Ok(total)
    }

    /// 获取内存分配摘要
    pub fn get_memory_summary(&self) -> String {
        let total_bytes = self.get_total_memory();
        let (value, unit) = MemoryUnit::from_bytes(total_bytes);

        let mut summary = String::new();
        summary.push_str(&format!("Total NUMA memory: {} {:?}\n", value, unit));

        for (i, cell) in self.cells.iter().enumerate() {
            let cell_bytes = cell.get_memory_bytes();
            let (cell_value, cell_unit) = MemoryUnit::from_bytes(cell_bytes);
            let cpu_count = cell.get_cpu_count().unwrap_or(0);

            summary.push_str(&format!(
                "  Cell {}: {} {:?} memory, {} CPUs\n",
                i, cell_value, cell_unit, cpu_count
            ));
        }

        summary
    }

    /// 获取距离矩阵
    pub fn get_distance_matrix(&self) -> Vec<Vec<u32>> {
        let total_cells = self.cells.len();
        let mut matrix = vec![vec![20u32; total_cells]; total_cells];

        // 修复：使用 enumerate() 而不是 range loop
        for (i, row) in matrix.iter_mut().enumerate() {
            row[i] = 10; // 本地距离

            if let Some(distances) = &self.cells[i].distances {
                for sibling in &distances.siblings {
                    let j = sibling.cell_id as usize;
                    if j < total_cells {
                        row[j] = sibling.value;
                    }
                }
            }
        }

        // 确保对称
        for (i, _cell) in self.cells.iter().enumerate() {
            for j in (i + 1)..total_cells {
                if matrix[i][j] != matrix[j][i] {
                    // 如果不一致，取最大值
                    let max_val = matrix[i][j].max(matrix[j][i]);
                    matrix[i][j] = max_val;
                    matrix[j][i] = max_val;
                }
            }
        }

        matrix
    }

    /// 获取建议的NUMA配置
    pub fn suggest_configuration(total_vcpus: u32, total_memory_gb: u32) -> Self {
        let mut topology = NumaTopology::new();

        // 简单的建议算法：根据CPU数量和内存大小决定NUMA节点数
        let suggested_nodes = if total_vcpus <= 8 {
            1
        } else if total_vcpus <= 32 {
            2
        } else if total_vcpus <= 64 {
            4
        } else {
            (total_vcpus as f32 / 16.0).ceil() as u32
        };

        let memory_per_node_gb = total_memory_gb / suggested_nodes;
        let cpus_per_node = total_vcpus / suggested_nodes;
        let remaining_cpus = total_vcpus % suggested_nodes;

        let mut cpu_offset = 0;

        for node_id in 0..suggested_nodes {
            let mut cpus_for_node = cpus_per_node;

            // 将剩余的CPU分配给第一个节点
            if node_id == 0 {
                cpus_for_node += remaining_cpus;
            }

            // 创建CPU集合字符串
            let cpu_start = cpu_offset;
            let cpu_end = cpu_offset + cpus_for_node - 1;
            let cpus_str = if cpu_start == cpu_end {
                format!("{}", cpu_start)
            } else {
                format!("{}-{}", cpu_start, cpu_end)
            };

            let cell = NumaCell::new(memory_per_node_gb as u64 * 1024 * 1024) // GB to KiB
                .with_id(node_id)
                .with_cpus(&cpus_str)
                .with_unit(MemoryUnit::GiB);

            topology = topology.add_cell(cell);
            cpu_offset += cpus_for_node;
        }

        topology
    }
}
