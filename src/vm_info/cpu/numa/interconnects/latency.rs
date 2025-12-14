use super::InterconnectType;
use serde::{Deserialize, Serialize};
/// 延迟配置
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LatencyConfig {
    #[serde(rename = "@initiator")]
    pub initiator: u32,

    #[serde(rename = "@target")]
    pub target: u32,

    #[serde(rename = "@type")]
    pub latency_type: InterconnectType,

    #[serde(rename = "@value")]
    pub value: u64, // 纳秒

    #[serde(rename = "@cache", skip_serializing_if = "Option::is_none")]
    pub cache_level: Option<u32>,
}

impl LatencyConfig {
    pub fn new(initiator: u32, target: u32, latency_type: InterconnectType, value: u64) -> Self {
        Self {
            initiator,
            target,
            latency_type,
            value,
            cache_level: None,
        }
    }

    pub fn with_cache_level(mut self, cache_level: u32) -> Self {
        self.cache_level = Some(cache_level);
        self
    }

    pub fn validate(&self, total_cells: u32) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.initiator >= total_cells {
            errors.push(format!(
                "Initiator cell {} out of range (0-{})",
                self.initiator,
                total_cells - 1
            ));
        }

        if self.target >= total_cells {
            errors.push(format!(
                "Target cell {} out of range (0-{})",
                self.target,
                total_cells - 1
            ));
        }

        if self.value == 0 {
            errors.push("Latency value cannot be 0".to_string());
        }

        if let Some(cache_level) = self.cache_level
            && (cache_level == 0 || cache_level > 4)
        {
            errors.push(format!("Invalid cache level: {}", cache_level));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
