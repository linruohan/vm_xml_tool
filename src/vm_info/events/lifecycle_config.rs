/// 生命周期事件配置
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct LifecycleConfig {
    #[serde(rename = "on_poweroff", skip_serializing_if = "Option::is_none")]
    pub on_poweroff: Option<LifecycleAction>,

    #[serde(rename = "on_reboot", skip_serializing_if = "Option::is_none")]
    pub on_reboot: Option<LifecycleAction>,

    #[serde(rename = "on_crash", skip_serializing_if = "Option::is_none")]
    pub on_crash: Option<LifecycleAction>,

    #[serde(rename = "on_lockfailure", skip_serializing_if = "Option::is_none")]
    pub on_lockfailure: Option<LifecycleAction>,
}

impl LifecycleConfig {
    /// 创建新的生命周期配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置关机事件动作
    pub fn on_poweroff(mut self, action: LifecycleAction) -> Self {
        self.on_poweroff = Some(action);
        self
    }

    /// 设置重启事件动作
    pub fn on_reboot(mut self, action: LifecycleAction) -> Self {
        self.on_reboot = Some(action);
        self
    }

    /// 设置崩溃事件动作
    pub fn on_crash(mut self, action: LifecycleAction) -> Self {
        self.on_crash = Some(action);
        self
    }

    /// 设置锁定失败事件动作
    pub fn on_lockfailure(mut self, action: LifecycleAction) -> Self {
        self.on_lockfailure = Some(action);
        self
    }

    /// 获取指定事件的配置
    pub fn get_action(&self, event: LifecycleEvent) -> Option<LifecycleAction> {
        match event {
            LifecycleEvent::OnPoweroff => self.on_poweroff,
            LifecycleEvent::OnReboot => self.on_reboot,
            LifecycleEvent::OnCrash => self.on_crash,
            LifecycleEvent::OnLockfailure => self.on_lockfailure,
        }
    }

    /// 设置指定事件的配置
    pub fn set_action(&mut self, event: LifecycleEvent, action: LifecycleAction) {
        match event {
            LifecycleEvent::OnPoweroff => self.on_poweroff = Some(action),
            LifecycleEvent::OnReboot => self.on_reboot = Some(action),
            LifecycleEvent::OnCrash => self.on_crash = Some(action),
            LifecycleEvent::OnLockfailure => self.on_lockfailure = Some(action),
        }
    }

    /// 清除指定事件的配置
    pub fn clear_action(&mut self, event: LifecycleEvent) {
        match event {
            LifecycleEvent::OnPoweroff => self.on_poweroff = None,
            LifecycleEvent::OnReboot => self.on_reboot = None,
            LifecycleEvent::OnCrash => self.on_crash = None,
            LifecycleEvent::OnLockfailure => self.on_lockfailure = None,
        }
    }

    /// 验证配置
    pub fn validate(&self, driver: Option<&str>) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // 检查动作的有效性
        self.validate_actions(&mut errors);

        // 检查驱动兼容性
        if let Some(driver_name) = driver {
            self.validate_driver_compatibility(driver_name, &mut errors);
        }

        // 检查特定组合限制
        self.validate_action_combinations(&mut errors);

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// 验证动作的有效性
    fn validate_actions(&self, errors: &mut Vec<String>) {
        let checks = vec![
            (LifecycleEvent::OnPoweroff, self.on_poweroff),
            (LifecycleEvent::OnReboot, self.on_reboot),
            (LifecycleEvent::OnCrash, self.on_crash),
            (LifecycleEvent::OnLockfailure, self.on_lockfailure),
        ];

        for (event, action_opt) in checks {
            if let Some(action) = action_opt {
                if !event.is_action_valid(action) {
                    errors.push(format!(
                        "Invalid action '{}' for event '{}'. Allowed actions: {}",
                        action,
                        event,
                        event.allowed_actions()
                             .iter()
                             .map(|a| a.to_string())
                             .collect::<Vec<_>>()
                             .join(", ")
                    ));
                }

                // 检查特定事件的动作限制
                if event == LifecycleEvent::OnCrash && action.is_lock_failure_only() {
                    errors.push(format!(
                        "Action '{}' is only valid for lockfailure events, not for crash events",
                        action
                    ));
                }

                if event == LifecycleEvent::OnLockfailure && action.is_crash_only() {
                    errors.push(format!(
                        "Action '{}' is only valid for crash events, not for lockfailure events",
                        action
                    ));
                }
            }
        }
    }

    /// 验证驱动兼容性
    fn validate_driver_compatibility(&self, driver: &str, errors: &mut Vec<String>) {
        let driver_lower = driver.to_lowercase();

        let checks = vec![
            (LifecycleEvent::OnPoweroff, self.on_poweroff),
            (LifecycleEvent::OnReboot, self.on_reboot),
            (LifecycleEvent::OnCrash, self.on_crash),
            (LifecycleEvent::OnLockfailure, self.on_lockfailure),
        ];

        for (event, action_opt) in checks {
            if let Some(action) = action_opt {
                if !action.is_supported_by_driver(&driver_lower) {
                    errors.push(format!(
                        "Action '{}' for event '{}' is not supported by driver '{}'",
                        action, event, driver
                    ));
                }

                // QEMU/KVM/HVF 特定限制
                if matches!(driver_lower.as_str(), "qemu" | "kvm" | "hvf") {
                    if matches!(event, LifecycleEvent::OnPoweroff | LifecycleEvent::OnReboot) {
                        if matches!(action, LifecycleAction::Preserve | LifecycleAction::RenameRestart) {
                            errors.push(format!(
                                "QEMU/KVM/HVF driver does not support action '{}' for event '{}'",
                                action, event
                            ));
                        }
                    }
                }
            }
        }
    }

    /// 验证动作组合限制
    fn validate_action_combinations(&self, errors: &mut Vec<String>) {
        // QEMU/KVM/HVF 限制：on_poweroff=restart 和 on_reboot=destroy 的组合是禁止的
        if let (Some(poweroff_action), Some(reboot_action)) = (self.on_poweroff, self.on_reboot) {
            if poweroff_action == LifecycleAction::Restart
                && reboot_action == LifecycleAction::Destroy {
                errors.push(
                    "QEMU/KVM/HVF: The combination of on_poweroff=restart and on_reboot=destroy is forbidden".to_string()
                );
            }
        }

        // 检查安装场景的常见配置
        if self.on_reboot == Some(LifecycleAction::Destroy)
            && self.on_poweroff != Some(LifecycleAction::Destroy) {
            errors.push(
                "Warning: Unusual configuration. When on_reboot=destroy, typically on_poweroff should also be destroy".to_string()
            );
        }
    }

    /// 获取配置摘要
    pub fn get_summary(&self) -> String {
        let mut summary = String::from("Lifecycle Event Configuration:\n");

        let events = vec![
            (LifecycleEvent::OnPoweroff, self.on_poweroff),
            (LifecycleEvent::OnReboot, self.on_reboot),
            (LifecycleEvent::OnCrash, self.on_crash),
            (LifecycleEvent::OnLockfailure, self.on_lockfailure),
        ];

        for (event, action_opt) in events {
            summary.push_str(&format!("  {}: ", event));

            if let Some(action) = action_opt {
                summary.push_str(&format!("{} - {}\n", action, action.description()));
            } else {
                summary.push_str("(use hypervisor default)\n");
            }
        }

        summary
    }

    /// 获取默认配置（基于最佳实践）
    pub fn default_for_os_installation() -> Self {
        // 操作系统安装期间的默认配置
        Self::new()
            .on_poweroff(LifecycleAction::Destroy)
            .on_reboot(LifecycleAction::Destroy)  // 在安装过程中，重启被视为关机
    }

    /// 获取生产环境默认配置
    pub fn default_for_production() -> Self {
        Self::new()
            .on_poweroff(LifecycleAction::Destroy)
            .on_reboot(LifecycleAction::Restart)
            .on_crash(LifecycleAction::Restart)
            .on_lockfailure(LifecycleAction::Restart)
    }

    /// 获取开发环境默认配置
    pub fn default_for_development() -> Self {
        Self::new()
            .on_poweroff(LifecycleAction::Destroy)
            .on_reboot(LifecycleAction::Restart)
            .on_crash(LifecycleAction::Preserve)  // 保留崩溃环境用于调试
            .on_lockfailure(LifecycleAction::Pause)  // 暂停以便调试锁定问题
    }

    /// 获取高可用性配置
    pub fn high_availability_config() -> Self {
        Self::new()
            .on_poweroff(LifecycleAction::Restart)  // 自动重启
            .on_reboot(LifecycleAction::Restart)
            .on_crash(LifecycleAction::CoredumpRestart)  // 转储核心后重启
            .on_lockfailure(LifecycleAction::Restart)  // 重启以重新获取锁
    }

    /// 获取调试配置
    pub fn debug_config() -> Self {
        Self::new()
            .on_poweroff(LifecycleAction::Preserve)
            .on_reboot(LifecycleAction::Preserve)
            .on_crash(LifecycleAction::Preserve)
            .on_lockfailure(LifecycleAction::Pause)
    }
}
