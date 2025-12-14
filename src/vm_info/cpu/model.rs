use crate::vm_info::cpu::cpu_mode::FallbackPolicy;
use serde::{Deserialize, Serialize};

/// CPU 模型配置
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct CpuModel {
    #[serde(rename = "$value")]
    pub name: String,

    #[serde(rename = "@fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<FallbackPolicy>,

    #[serde(rename = "@vendor_id", skip_serializing_if = "Option::is_none")]
    pub vendor_id: Option<String>,
}

impl CpuModel {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            fallback: None,
            vendor_id: None,
        }
    }

    pub fn with_fallback(mut self, fallback: FallbackPolicy) -> Self {
        self.fallback = Some(fallback);
        self
    }

    pub fn with_vendor_id(mut self, vendor_id: &str) -> Self {
        if vendor_id.len() == 12 {
            self.vendor_id = Some(vendor_id.to_string());
        } else {
            // 如果长度不对，不设置
            eprintln!(
                "Warning: vendor_id must be exactly 12 characters, got '{}' ({} chars)",
                vendor_id,
                vendor_id.len()
            );
        }
        self
    }

    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.name.is_empty() {
            errors.push("CPU model name cannot be empty".to_string());
        }

        if let Some(vendor_id) = &self.vendor_id
            && vendor_id.len() != 12
        {
            errors.push(format!(
                "vendor_id must be exactly 12 characters, got '{}' ({} chars)",
                vendor_id,
                vendor_id.len()
            ));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
