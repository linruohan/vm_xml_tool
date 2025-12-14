use serde::{Deserialize, Serialize};
use std::fmt;

/// 缓存模式
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum CacheMode {
    Emulate,
    Passthrough,
    Disable,
}

impl fmt::Display for CacheMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CacheMode::Emulate => write!(f, "emulate"),
            CacheMode::Passthrough => write!(f, "passthrough"),
            CacheMode::Disable => write!(f, "disable"),
        }
    }
}

/// CPU 缓存配置
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct CpuCache {
    #[serde(rename = "@mode")]
    pub mode: CacheMode,

    #[serde(rename = "@level", skip_serializing_if = "Option::is_none")]
    pub level: Option<u32>,
}

impl CpuCache {
    pub fn new(mode: CacheMode) -> Self {
        Self { mode, level: None }
    }

    pub fn with_level(mut self, level: u32) -> Self {
        if (1..=4).contains(&level) {
            self.level = Some(level);
        }
        self
    }

    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if let Some(level) = self.level
            && (!(1..=4).contains(&level))
        {
            errors.push(format!(
                "Cache level must be between 1 and 4, got {}",
                level
            ));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
