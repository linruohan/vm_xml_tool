use serde::{Deserialize, Serialize};
use std::fmt;

/// 最大物理地址模式
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum MaxPhysAddrMode {
    Passthrough,
    Emulate,
}

impl fmt::Display for MaxPhysAddrMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MaxPhysAddrMode::Passthrough => write!(f, "passthrough"),
            MaxPhysAddrMode::Emulate => write!(f, "emulate"),
        }
    }
}

/// 最大物理地址配置
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct MaxPhysAddr {
    #[serde(rename = "@mode")]
    pub mode: MaxPhysAddrMode,

    #[serde(rename = "@bits", skip_serializing_if = "Option::is_none")]
    pub bits: Option<u32>,

    #[serde(rename = "@limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl MaxPhysAddr {
    pub fn new(mode: MaxPhysAddrMode) -> Self {
        Self {
            mode,
            bits: None,
            limit: None,
        }
    }

    pub fn with_bits(mut self, bits: u32) -> Self {
        self.bits = Some(bits);
        self
    }

    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        match self.mode {
            MaxPhysAddrMode::Emulate => {
                if self.bits.is_none() {
                    errors.push("bits attribute is required for emulate mode".to_string());
                }

                if let Some(bits) = self.bits {
                    if bits > 52 {
                        // 当前最大物理地址位数
                        errors.push(format!(
                            "bits ({}) exceeds maximum supported value (52)",
                            bits
                        ));
                    }
                    if bits < 32 {
                        errors.push(format!("bits ({}) is too small for modern systems", bits));
                    }
                }

                if self.limit.is_some() {
                    errors.push("limit attribute is not used in emulate mode".to_string());
                }
            }
            MaxPhysAddrMode::Passthrough => {
                if self.bits.is_some() {
                    errors.push("bits attribute is not used in passthrough mode".to_string());
                }

                if let Some(limit) = self.limit
                    && limit > 52 {
                        errors.push(format!(
                            "limit ({}) exceeds maximum supported value (52)",
                            limit
                        ));
                    }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
