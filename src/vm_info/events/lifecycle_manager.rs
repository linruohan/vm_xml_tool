use std::collections::HashMap;

/// 生命周期配置管理器
#[derive(Debug, Default)]
pub struct LifecycleManager {
    /// 支持的驱动及其能力
    driver_capabilities: HashMap<String, DriverCapabilities>,
    /// 配置历史
    config_history: Vec<LifecycleConfig>,
}

/// 驱动能力
#[derive(Debug, Clone)]
pub struct DriverCapabilities {
    pub name: String,
    pub supports_poweroff: bool,
    pub supports_reboot: bool,
    pub supports_crash: bool,
    pub supports_lockfailure: bool,
    pub supported_actions: Vec<LifecycleAction>,
}

impl Default for DriverCapabilities {
    fn default() -> Self {
        Self {
            name: "generic".to_string(),
            supports_poweroff: true,
            supports_reboot: true,
            supports_crash: true,
            supports_lockfailure: true,
            supported_actions: vec![
                LifecycleAction::Destroy,
                LifecycleAction::Restart,
                LifecycleAction::Preserve,
            ],
        }
    }
}

impl LifecycleManager {
    /// 创建新的生命周期管理器
    pub fn new() -> Self {
        let mut manager = Self::default();
        manager.init_default_capabilities();
        manager
    }

    /// 初始化默认的驱动能力
    fn init_default_capabilities(&mut self) {
        // QEMU/KVM/HVF 驱动
        self.driver_capabilities.insert("qemu".to_string(), DriverCapabilities {
            name: "qemu".to_string(),
            supports_poweroff: true,
            supports_reboot: true,
            supports_crash: true,
            supports_lockfailure: true,
            supported_actions: vec![
                LifecycleAction::Destroy,
                LifecycleAction::Restart,
                LifecycleAction::CoredumpDestroy,
                LifecycleAction::CoredumpRestart,
            ],
        });

        // Xen/libxl 驱动
        self.driver_capabilities.insert("libxl".to_string(), DriverCapabilities {
            name: "libxl".to_string(),
            supports_poweroff: true,
            supports_reboot: true,
            supports_crash: true,
            supports_lockfailure: true,
            supported_actions: vec![
                LifecycleAction::Destroy,
                LifecycleAction::Restart,
                LifecycleAction::Preserve,
                LifecycleAction::RenameRestart,
                LifecycleAction::CoredumpDestroy,
                LifecycleAction::CoredumpRestart,
            ],
        });

        // VMware 驱动
        self.driver_capabilities.insert("vmware".to_string(), DriverCapabilities {
            name: "vmware".to_string(),
            supports_poweroff: true,
            supports_reboot: true,
            supports_crash: false,  // VMware 可能不支持崩溃事件
            supports_lockfailure: false,
            supported_actions: vec![
                LifecycleAction::Destroy,
                LifecycleAction::Restart,
            ],
        });

        // Hyper-V 驱动
        self.driver_capabilities.insert("hyperv".to_string(), DriverCapabilities {
            name: "hyperv".to_string(),
            supports_poweroff: true,
            supports_reboot: true,
            supports_crash: false,
            supports_lockfailure: false,
            supported_actions: vec![
                LifecycleAction::Destroy,
                LifecycleAction::Restart,
            ],
        });
    }

    /// 验证配置（考虑驱动能力）
    pub fn validate_config(&self, config: &LifecycleConfig, driver: &str) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // 基础验证
        match config.validate(Some(driver)) {
            Ok(_) => {}
            Err(mut config_errors) => {
                errors.append(&mut config_errors);
            }
        }

        // 驱动能力验证
        if let Some(capabilities) = self.driver_capabilities.get(driver) {
            // 检查事件是否受支持
            if config.on_poweroff.is_some() && !capabilities.supports_poweroff {
                errors.push(format!("Driver '{}' does not support on_poweroff events", driver));
            }

            if config.on_reboot.is_some() && !capabilities.supports_reboot {
                errors.push(format!("Driver '{}' does not support on_reboot events", driver));
            }

            if config.on_crash.is_some() && !capabilities.supports_crash {
                errors.push(format!("Driver '{}' does not support on_crash events", driver));
            }

            if config.on_lockfailure.is_some() && !capabilities.supports_lockfailure {
                errors.push(format!("Driver '{}' does not support on_lockfailure events", driver));
            }

            // 检查动作是否受支持
            let checks = vec![
                ("on_poweroff", config.on_poweroff),
                ("on_reboot", config.on_reboot),
                ("on_crash", config.on_crash),
                ("on_lockfailure", config.on_lockfailure),
            ];

            for (event_name, action_opt) in checks {
                if let Some(action) = action_opt {
                    if !capabilities.supported_actions.contains(&action) {
                        errors.push(format!(
                            "Driver '{}' does not support action '{}' for {}",
                            driver, action, event_name
                        ));
                    }
                }
            }
        } else {
            errors.push(format!("Unknown driver '{}'", driver));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// 获取建议的配置（基于场景）
    pub fn get_suggested_config(&self, scenario: &str, driver: &str) -> Option<LifecycleConfig> {
        match scenario.to_lowercase().as_str() {
            "os-installation" => Some(LifecycleConfig::default_for_os_installation()),
            "production" => Some(LifecycleConfig::default_for_production()),
            "development" => Some(LifecycleConfig::default_for_development()),
            "high-availability" => Some(LifecycleConfig::high_availability_config()),
            "debugging" => Some(LifecycleConfig::debug_config()),
            _ => None,
        }.and_then(|config| {
            // 确保配置与驱动兼容
            if self.validate_config(&config, driver).is_ok() {
                Some(config)
            } else {
                None
            }
        })
    }

    /// 记录配置历史
    pub fn record_config(&mut self, config: LifecycleConfig) {
        self.config_history.push(config);
    }

    /// 获取配置历史摘要
    pub fn get_history_summary(&self) -> String {
        let mut summary = format!("Lifecycle Configuration History ({} entries):\n", self.config_history.len());

        for (i, config) in self.config_history.iter().enumerate() {
            summary.push_str(&format!("\nEntry {}:\n", i + 1));
            summary.push_str(&config.get_summary());
        }

        summary
    }

    /// 获取驱动能力摘要
    pub fn get_driver_summary(&self) -> String {
        let mut summary = String::from("Driver Capabilities:\n");

        for (name, capabilities) in &self.driver_capabilities {
            summary.push_str(&format!("\nDriver: {}\n", name));
            summary.push_str(&format!("  Supported events:\n"));
            if capabilities.supports_poweroff {
                summary.push_str("    - on_poweroff\n");
            }
            if capabilities.supports_reboot {
                summary.push_str("    - on_reboot\n");
            }
            if capabilities.supports_crash {
                summary.push_str("    - on_crash\n");
            }
            if capabilities.supports_lockfailure {
                summary.push_str("    - on_lockfailure\n");
            }

            summary.push_str(&format!("  Supported actions:\n"));
            for action in &capabilities.supported_actions {
                summary.push_str(&format!("    - {} ({})\n", action, action.description()));
            }
        }

        summary
    }
}