use serde::{Deserialize, Serialize};
use std::fmt;

/// CPU 特性策略
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum FeaturePolicy {
    Force,
    #[default]
    Require,
    Optional,
    Disable,
    Forbid,
}

impl fmt::Display for FeaturePolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FeaturePolicy::Force => write!(f, "force"),
            FeaturePolicy::Require => write!(f, "require"),
            FeaturePolicy::Optional => write!(f, "optional"),
            FeaturePolicy::Disable => write!(f, "disable"),
            FeaturePolicy::Forbid => write!(f, "forbid"),
        }
    }
}

/// CPU 特性配置
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct CpuFeature {
    #[serde(rename = "@policy")]
    pub policy: FeaturePolicy,

    #[serde(rename = "@name")]
    pub name: String,
}

impl CpuFeature {
    pub fn new(name: &str, policy: FeaturePolicy) -> Self {
        Self {
            name: name.to_string(),
            policy,
        }
    }

    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.name.is_empty() {
            errors.push("CPU feature name cannot be empty".to_string());
        }

        // 检查特性名称格式（简单的检查）
        if self
            .name
            .contains(|c: char| c.is_whitespace() || c.is_control())
        {
            errors.push(format!(
                "CPU feature name contains invalid characters: {}",
                self.name
            ));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
