use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// NUMA 节点距离配置
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NumaDistance {
    #[serde(rename = "@id")]
    pub cell_id: u32,

    #[serde(rename = "@value")]
    pub value: u32,
}

impl NumaDistance {
    pub fn new(cell_id: u32, value: u32) -> Self {
        Self { cell_id, value }
    }

    pub fn validate(&self, max_cells: u32) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.cell_id >= max_cells {
            errors.push(format!(
                "Distance references invalid cell id: {} (max: {})",
                self.cell_id,
                max_cells - 1
            ));
        }

        if self.value < 10 {
            errors.push(format!(
                "Distance value {} is too small, minimum is 10",
                self.value
            ));
        }

        if self.value > 255 {
            errors.push(format!(
                "Distance value {} exceeds maximum (255)",
                self.value
            ));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

/// 距离表配置
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct DistanceTable {
    #[serde(rename = "sibling")]
    pub siblings: Vec<NumaDistance>,
}

impl DistanceTable {
    pub fn new() -> Self {
        Self {
            siblings: Vec::new(),
        }
    }

    pub fn add_sibling(mut self, sibling: NumaDistance) -> Self {
        self.siblings.push(sibling);
        self
    }

    pub fn with_siblings(mut self, siblings: Vec<NumaDistance>) -> Self {
        self.siblings = siblings;
        self
    }

    pub fn validate(&self, cell_id: u32, total_cells: u32) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // 检查自引用
        if let Some(self_ref) = self.siblings.iter().find(|s| s.cell_id == cell_id) {
            if self_ref.value != 10 {
                errors.push(format!(
                    "Self-distance for cell {} should be 10, got {}",
                    cell_id, self_ref.value
                ));
            }
        } else {
            // 缺少自引用
            errors.push(format!(
                "Missing self-distance for cell {} (should have value 10)",
                cell_id
            ));
        }

        // 验证每个距离
        for sibling in &self.siblings {
            match sibling.validate(total_cells) {
                Ok(_) => {}
                Err(mut sibling_errors) => {
                    errors.append(&mut sibling_errors);
                }
            }
        }

        // 检查重复的 cell_id
        let mut seen = HashSet::new();
        for sibling in &self.siblings {
            if !seen.insert(sibling.cell_id) {
                errors.push(format!(
                    "Duplicate distance entry for cell {} in distance table of cell {}",
                    sibling.cell_id, cell_id
                ));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// 获取距离矩阵
    pub fn get_distance_matrix(&self, _total_cells: u32) -> HashMap<(u32, u32), u32> {
        let mut matrix = HashMap::new();

        // 假设这是某个 cell 的距离表
        for sibling in &self.siblings {
            matrix.insert((0, sibling.cell_id), sibling.value);
        }

        matrix
    }
}
