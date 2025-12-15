/// 生命周期事件类型
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Hash)]
#[serde(rename_all = "lowercase")]
pub enum LifecycleEvent {
    /// 关机事件
    #[serde(rename = "on_poweroff")]
    OnPoweroff,
    /// 重启事件
    #[serde(rename = "on_reboot")]
    OnReboot,
    /// 崩溃事件
    #[serde(rename = "on_crash")]
    OnCrash,
    /// 锁定失败事件
    #[serde(rename = "on_lockfailure")]
    OnLockfailure,
}

impl LifecycleEvent {
    /// 获取事件描述
    pub fn description(&self) -> &'static str {
        match self {
            LifecycleEvent::OnPoweroff => "Guest requests a poweroff",
            LifecycleEvent::OnReboot => "Guest requests a reboot",
            LifecycleEvent::OnCrash => "Guest crashes",
            LifecycleEvent::OnLockfailure => "Lock manager loses resource locks",
        }
    }

    /// 获取允许的动作列表
    pub fn allowed_actions(&self) -> Vec<LifecycleAction> {
        match self {
            LifecycleEvent::OnPoweroff | LifecycleEvent::OnReboot => {
                vec![
                    LifecycleAction::Destroy,
                    LifecycleAction::Restart,
                    LifecycleAction::Preserve,
                    LifecycleAction::RenameRestart,
                ]
            }
            LifecycleEvent::OnCrash => {
                vec![
                    LifecycleAction::Destroy,
                    LifecycleAction::Restart,
                    LifecycleAction::Preserve,
                    LifecycleAction::RenameRestart,
                    LifecycleAction::CoredumpDestroy,
                    LifecycleAction::CoredumpRestart,
                ]
            }
            LifecycleEvent::OnLockfailure => {
                vec![
                    LifecycleAction::Poweroff,
                    LifecycleAction::Restart,
                    LifecycleAction::Pause,
                    LifecycleAction::Ignore,
                ]
            }
        }
    }

    /// 检查动作是否对该事件有效
    pub fn is_action_valid(&self, action: LifecycleAction) -> bool {
        self.allowed_actions().contains(&action)
    }

    /// 获取事件触发的来源
    pub fn trigger_sources(&self) -> Vec<&'static str> {
        match self {
            LifecycleEvent::OnPoweroff => vec![
                "virDomainShutdown API",
                "virDomainShutdownFlags API",
                "virsh shutdown command",
                "Guest OS poweroff request",
            ],
            LifecycleEvent::OnReboot => vec![
                "virDomainReboot API",
                "virsh reboot command",
                "Guest OS reboot request",
            ],
            LifecycleEvent::OnCrash => vec![
                "Guest OS crash",
                "Hypervisor crash detection",
            ],
            LifecycleEvent::OnLockfailure => vec![
                "Lock manager loses resource locks",
                "Distributed lock management failure",
            ],
        }
    }
}

impl fmt::Display for LifecycleEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LifecycleEvent::OnPoweroff => write!(f, "on_poweroff"),
            LifecycleEvent::OnReboot => write!(f, "on_reboot"),
            LifecycleEvent::OnCrash => write!(f, "on_crash"),
            LifecycleEvent::OnLockfailure => write!(f, "on_lockfailure"),
        }
    }
}