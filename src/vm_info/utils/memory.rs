use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// 内存单位枚举
#[derive(Debug, Serialize, Deserialize, PartialEq, Default, Clone, Copy)]
pub enum MemoryUnit {
    #[serde(rename = "bytes")]
    Bytes,
    #[serde(rename = "KiB")]
    #[default]
    KiB,
    #[serde(rename = "MiB")]
    MiB,
    #[serde(rename = "GiB")]
    GiB,
    #[serde(rename = "TiB")]
    TiB,
    #[serde(rename = "K")]
    K, // 兼容旧格式
    #[serde(rename = "M")]
    M,
    #[serde(rename = "G")]
    G,
    #[serde(rename = "T")]
    T,
}

impl FromStr for MemoryUnit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "bytes" => Ok(MemoryUnit::Bytes),
            "kib" => Ok(MemoryUnit::KiB),
            "k" => Ok(MemoryUnit::K),
            "mib" => Ok(MemoryUnit::MiB),
            "m" => Ok(MemoryUnit::M),
            "gib" => Ok(MemoryUnit::GiB),
            "g" => Ok(MemoryUnit::G),
            "tib" => Ok(MemoryUnit::TiB),
            "t" => Ok(MemoryUnit::T),
            _ => Err(format!("未知的内存单位: {}", s)),
        }
    }
}

/// 带单位的内存值
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct MemoryValue {
    #[serde(rename = "$value")]
    pub value: u64,
    #[serde(rename = "@unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<MemoryUnit>,
}

impl MemoryValue {
    /// 创建新的内存值
    pub fn new(value: u64, unit: MemoryUnit) -> Self {
        Self {
            value,
            unit: Some(unit),
        }
    }

    /// 转换为 KiB（libvirt 输出标准）
    pub fn to_kib(&self) -> u64 {
        let unit = self.unit.unwrap_or_default();
        match unit {
            MemoryUnit::Bytes => self.value / 1024,
            MemoryUnit::KiB | MemoryUnit::K => self.value,
            MemoryUnit::MiB | MemoryUnit::M => self.value * 1024,
            MemoryUnit::GiB | MemoryUnit::G => self.value * 1024 * 1024,
            MemoryUnit::TiB | MemoryUnit::T => self.value * 1024 * 1024 * 1024,
            MemoryUnit::TiB | MemoryUnit::T => self.value * 1024 * 1024 * 1024,
        }
    }

    /// 从 KiB 创建
    pub fn from_kib(kib: u64) -> Self {
        Self {
            value: kib,
            unit: Some(MemoryUnit::KiB),
        }
    }

    /// 获取人类可读的字符串
    pub fn to_human_readable(&self) -> String {
        let (value, unit_str) = match self.unit {
            Some(MemoryUnit::Bytes) => (self.value as f64, "bytes"),
            Some(MemoryUnit::KiB | MemoryUnit::K) => (self.value as f64, "KiB"),
            Some(MemoryUnit::MiB | MemoryUnit::M) => (self.value as f64, "MiB"),
            Some(MemoryUnit::GiB | MemoryUnit::G) => (self.value as f64, "GiB"),
            Some(MemoryUnit::TiB | MemoryUnit::T) => (self.value as f64, "TiB"),
            None => (self.value as f64, "KiB"),
        };
        format!("{} {}", value, unit_str)
    }
}
