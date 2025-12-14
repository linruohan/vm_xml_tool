use serde::{Deserialize, Serialize};
use std::fmt;

/// CPU 模式
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
#[derive(Default)]
pub enum CpuMode {
    #[default]
    Custom,
    HostModel,
    HostPassthrough,
    Maximum,
}


impl fmt::Display for CpuMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CpuMode::Custom => write!(f, "custom"),
            CpuMode::HostModel => write!(f, "host-model"),
            CpuMode::HostPassthrough => write!(f, "host-passthrough"),
            CpuMode::Maximum => write!(f, "maximum"),
        }
    }
}

/// CPU 模型回退策略
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum FallbackPolicy {
    #[default]
    Allow,
    Forbid,
}

