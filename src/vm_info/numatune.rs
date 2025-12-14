use serde::{Deserialize, Serialize};

// 主结构体：NUMA 调优配置
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Numatune {
    #[serde(rename = "memory", skip_serializing_if = "Option::is_none")]
    pub memory: Option<MemoryTune>,

    #[serde(rename = "memnode", default)]
    pub mem_nodes: Vec<MemNode>,
}

// 内存调优配置
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MemoryTune {
    #[serde(rename = "@mode")]
    pub mode: MemoryMode,

    #[serde(rename = "@nodeset")]
    pub nodeset: String,

    #[serde(rename = "@placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<PlacementMode>,
}

// 内存节点配置
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MemNode {
    #[serde(rename = "@cellid")]
    pub cell_id: u32,

    #[serde(rename = "@mode")]
    pub mode: MemoryMode,

    #[serde(rename = "@nodeset")]
    pub nodeset: String,
}

// 内存分配模式枚举
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum MemoryMode {
    Interleave,
    Strict,
    Preferred,
    Restrictive,
}

// 内存放置模式枚举
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum PlacementMode {
    Static,
    Auto,
}
