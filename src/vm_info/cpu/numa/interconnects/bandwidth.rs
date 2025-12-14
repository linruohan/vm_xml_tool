use crate::MemoryUnit;
use crate::vm_info::cpu::numa::interconnects::InterconnectType;
use serde::{Deserialize, Serialize};

/// 带宽配置
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct BandwidthConfig {
    #[serde(rename = "@initiator")]
    pub initiator: u32,

    #[serde(rename = "@target")]
    pub target: u32,

    #[serde(rename = "@type")]
    pub bandwidth_type: InterconnectType,

    #[serde(rename = "@value")]
    pub value: u64, // KiB/s

    #[serde(rename = "@unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<MemoryUnit>,
}

impl BandwidthConfig {
    pub fn new(initiator: u32, target: u32, bandwidth_type: InterconnectType, value: u64) -> Self {
        Self {
            initiator,
            target,
            bandwidth_type,
            value,
            unit: None,
        }
    }

    pub fn with_unit(mut self, unit: MemoryUnit) -> Self {
        self.unit = Some(unit);
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
            errors.push("Bandwidth value cannot be 0".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// 转换为 KiB/s
    pub fn to_kib_per_sec(&self) -> u64 {
        match self.unit {
            Some(MemoryUnit::MiB) => self.value * 1024,
            Some(MemoryUnit::GiB) => self.value * 1024 * 1024,
            Some(MemoryUnit::TiB) => self.value * 1024 * 1024 * 1024,
            _ => self.value, // 默认为 KiB/s
        }
    }
}
