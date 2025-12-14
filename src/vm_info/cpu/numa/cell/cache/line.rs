use crate::MemoryUnit;
use serde::{Deserialize, Serialize};

/// 缓存行大小
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct CacheLine {
    #[serde(rename = "@value")]
    pub value: u64,

    #[serde(rename = "@unit")]
    pub unit: MemoryUnit,
}

impl Default for CacheLine {
    fn default() -> Self {
        Self {
            value: 64, // 64 bytes
            unit: MemoryUnit::Bytes,
        }
    }
}
