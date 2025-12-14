mod line;
mod size;
use line::CacheLine;
use size::CacheSize;

use crate::MemoryUnit;
use serde::{Deserialize, Serialize};

/// 缓存关联性
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Hash)]
#[serde(rename_all = "lowercase")]
pub enum CacheAssociativity {
    None,
    Direct,
    Full,
}

/// 缓存策略
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Hash)]
#[serde(rename_all = "lowercase")]
pub enum CachePolicy {
    None,
    Writeback,
    Writethrough,
}

/// 缓存配置
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct CacheConfig {
    /// 缓存级别
    #[serde(rename = "@level")]
    pub level: u32,

    /// 关联性
    #[serde(rename = "@associativity")]
    pub associativity: CacheAssociativity,

    /// 策略
    #[serde(rename = "@policy")]
    pub policy: CachePolicy,

    /// 缓存大小
    #[serde(rename = "size")]
    pub size: CacheSize,

    /// 缓存行大小
    #[serde(rename = "line")]
    pub line: CacheLine,
}

impl CacheConfig {
    pub fn new(level: u32, associativity: CacheAssociativity, policy: CachePolicy) -> Self {
        Self {
            level,
            associativity,
            policy,
            size: CacheSize::default(),
            line: CacheLine::default(),
        }
    }

    pub fn with_size(mut self, value: u64, unit: MemoryUnit) -> Self {
        self.size = CacheSize { value, unit };
        self
    }

    pub fn with_line(mut self, value: u64, unit: MemoryUnit) -> Self {
        self.line = CacheLine { value, unit };
        self
    }

    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.level == 0 {
            errors.push("Cache level must be greater than 0".to_string());
        }

        if self.level > 4 {
            errors.push(format!("Cache level {} is unusually high", self.level));
        }

        // 验证大小
        if self.size.value == 0 {
            errors.push("Cache size cannot be 0".to_string());
        }

        // 验证行大小
        if self.line.value == 0 {
            errors.push("Cache line size cannot be 0".to_string());
        }

        // 常见行大小检查
        let line_bytes = self.line.unit.to_bytes(self.line.value);
        if ![32, 64, 128, 256].contains(&line_bytes) {
            errors.push(format!("Unusual cache line size: {} bytes", line_bytes));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
