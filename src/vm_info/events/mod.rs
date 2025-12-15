use serde::{Deserialize, Serialize};
use std::fmt;

/// 生命周期动作枚举
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Hash)]
#[serde(rename_all = "lowercase")]
pub enum LifecycleAction {
    /// 销毁域并释放所有资源
    Destroy,
    /// 重启域（相同的配置）
    Restart,
    /// 保留资源用于分析
    Preserve,
    /// 重启域（使用新名称）- 仅限 libxl
    RenameRestart,
    /// 生成核心转储后销毁
    CoredumpDestroy,
    /// 生成核心转储后重启
    CoredumpRestart,
    /// 强制关机
    Poweroff,
    /// 暂停域
    Pause,
    /// 忽略事件
    Ignore,
}

impl LifecycleAction {
    /// 获取动作描述
    pub fn description(&self) -> &'static str {
        match self {
            LifecycleAction::Destroy => "Destroy the domain and release all resources",
            LifecycleAction::Restart => "Restart the domain with the same configuration",
            LifecycleAction::Preserve => "Preserve domain resources for analysis",
            LifecycleAction::RenameRestart => "Restart with a new name (libxl only)",
            LifecycleAction::CoredumpDestroy => "Core dump then destroy",
            LifecycleAction::CoredumpRestart => "Core dump then restart",
            LifecycleAction::Poweroff => "Forcefully power off the domain",
            LifecycleAction::Pause => "Pause the domain",
            LifecycleAction::Ignore => "Ignore the event",
        }
    }

    /// 检查动作是否仅用于崩溃事件
    pub fn is_crash_only(&self) -> bool {
        matches!(
            self,
            LifecycleAction::CoredumpDestroy | LifecycleAction::CoredumpRestart
        )
    }

    /// 检查动作是否仅用于锁定失败事件
    pub fn is_lock_failure_only(&self) -> bool {
        matches!(
            self,
            LifecycleAction::Poweroff | LifecycleAction::Pause | LifecycleAction::Ignore
        )
    }

    /// 检查动作是否受特定驱动支持
    pub fn is_supported_by_driver(&self, driver: &str) -> bool {
        match self {
            LifecycleAction::RenameRestart => driver == "libxl",
            _ => true,
        }
    }
}

impl fmt::Display for LifecycleAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LifecycleAction::Destroy => write!(f, "destroy"),
            LifecycleAction::Restart => write!(f, "restart"),
            LifecycleAction::Preserve => write!(f, "preserve"),
            LifecycleAction::RenameRestart => write!(f, "rename-restart"),
            LifecycleAction::CoredumpDestroy => write!(f, "coredump-destroy"),
            LifecycleAction::CoredumpRestart => write!(f, "coredump-restart"),
            LifecycleAction::Poweroff => write!(f, "poweroff"),
            LifecycleAction::Pause => write!(f, "pause"),
            LifecycleAction::Ignore => write!(f, "ignore"),
        }
    }
}
