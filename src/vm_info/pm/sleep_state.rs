use serde::{Deserialize, Serialize};
use std::fmt;

/// 睡眠状态枚举
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Hash, Default)]
#[serde(rename_all = "lowercase")]
pub enum SleepState {
    /// 挂起到内存 (S3)
    #[serde(rename = "suspend-to-mem")]
    #[default]
    SuspendToMem,
    /// 挂起到磁盘 (S4)
    #[serde(rename = "suspend-to-disk")]
    SuspendToDisk,
}

impl SleepState {
    /// 获取睡眠状态描述
    pub fn description(&self) -> &'static str {
        match self {
            SleepState::SuspendToMem => "Suspend to RAM (ACPI S3 state)",
            SleepState::SuspendToDisk => "Suspend to Disk (ACPI S4 state)",
        }
    }

    /// 获取 ACPI 状态标识
    pub fn acpi_state(&self) -> &'static str {
        match self {
            SleepState::SuspendToMem => "S3",
            SleepState::SuspendToDisk => "S4",
        }
    }

    /// 获取推荐的典型配置
    pub fn typical_setting(&self) -> bool {
        match self {
            SleepState::SuspendToMem => true,   // S3 通常启用
            SleepState::SuspendToDisk => false, // S4 通常禁用
        }
    }
}

impl fmt::Display for SleepState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SleepState::SuspendToMem => write!(f, "suspend-to-mem"),
            SleepState::SuspendToDisk => write!(f, "suspend-to-disk"),
        }
    }
}

/// 睡眠状态配置
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SleepStateConfig {
    /// 是否启用该睡眠状态
    #[serde(rename = "@enabled")]
    pub enabled: String,

    /// 睡眠状态类型（自动从元素名派生）
    #[serde(skip)]
    pub state: SleepState,
}
impl SleepStateConfig {
    /// 创建新的睡眠状态配置
    pub fn new(state: SleepState, enabled: String) -> Self {
        Self { enabled, state }
    }

    /// 启用睡眠状态
    pub fn enabled(state: SleepState) -> Self {
        Self::new(state, "yes".to_string())
    }

    /// 禁用睡眠状态
    pub fn disabled(state: SleepState) -> Self {
        Self::new(state, "no".to_string())
    }

    /// 获取配置摘要
    pub fn get_summary(&self) -> String {
        format!(
            "{} (ACPI {}): {} - {}",
            self.state,
            self.state.acpi_state(),
            if self.enabled == "yes" {
                "enabled"
            } else {
                "disabled"
            },
            self.state.description()
        )
    }
}
