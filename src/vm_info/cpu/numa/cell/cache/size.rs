use crate::MemoryUnit;
use serde::{Deserialize, Serialize};

/// 缓存大小
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct CacheSize {
    #[serde(rename = "@value")]
    pub value: u64,

    #[serde(rename = "@unit")]
    pub unit: MemoryUnit,
}

impl Default for CacheSize {
    fn default() -> Self {
        Self {
            value: 1024, // 1 KiB
            unit: MemoryUnit::KiB,
        }
    }
}
