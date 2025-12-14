use serde::{Deserialize, Serialize};
use std::fmt;

/// CPU 匹配模式
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum CpuMatch {
    Minimum,
    #[default]
    Exact,
    Strict,
}

impl fmt::Display for CpuMatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CpuMatch::Minimum => write!(f, "minimum"),
            CpuMatch::Exact => write!(f, "exact"),
            CpuMatch::Strict => write!(f, "strict"),
        }
    }
}
