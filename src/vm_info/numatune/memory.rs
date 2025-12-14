use super::NodeSet;
use super::NumaMode;
use serde::{Deserialize, Serialize};
// 内存调优配置
#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct NumaMemory {
    #[serde(rename = "@mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<NumaMode>,
    #[serde(rename = "@nodeset", skip_serializing_if = "Option::is_none")]
    pub nodeset: Option<NodeSet>,
    #[serde(rename = "@placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<PlacementMode>,
}

// 内存放置模式枚举
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy, Default)]
#[serde(rename_all = "lowercase")]
pub enum PlacementMode {
    #[default]
    /// 静态放置：使用显式指定的节点集
    Static,
    /// 自动放置：由numad自动选择最优节点
    Auto,
}

impl NumaMemory {
    /// 创建新的内存配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置模式
    pub fn with_mode(mut self, mode: NumaMode) -> Self {
        self.mode = Some(mode);
        self
    }

    /// 设置节点集
    pub fn with_nodeset(mut self, nodeset: &str) -> Self {
        self.nodeset = Some(NodeSet::new(nodeset));
        self
    }

    /// 设置放置模式
    pub fn with_placement(mut self, placement: PlacementMode) -> Self {
        self.placement = Some(placement);
        self
    }
}
