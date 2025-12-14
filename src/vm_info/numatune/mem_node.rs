use super::NodeSet;
use super::NumaMode;
use serde::{Deserialize, Serialize};

// 内存节点配置
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MemNode {
    #[serde(rename = "@cellid")]
    pub cell_id: u32,

    #[serde(rename = "@mode")]
    pub mode: NumaMode,

    #[serde(rename = "@nodeset")]
    pub nodeset: NodeSet,
}

impl MemNode {
    /// 创建新的节点配置
    pub fn new(cell_id: u32, mode: NumaMode, nodeset: &str) -> Self {
        Self {
            cell_id,
            mode,
            nodeset: NodeSet::new(nodeset),
        }
    }
}
