use super::PowerManagement;

/// 电源管理配置管理器
#[derive(Debug, Default)]
pub struct PowerManagementManager {
    /// 支持的驱动列表
    supported_drivers: Vec<String>,
    /// 配置历史
    config_history: Vec<PowerManagement>,
}

impl PowerManagementManager {
    /// 创建新的电源管理管理器
    pub fn new() -> Self {
        Self {
            supported_drivers: vec!["qemu".to_string()],
            config_history: Vec::new(),
        }
    }

    /// 检查驱动是否支持电源管理
    pub fn is_driver_supported(&self, driver: &str) -> bool {
        self.supported_drivers
            .iter()
            .any(|supported| supported == driver)
    }

    /// 验证配置（考虑驱动支持）
    pub fn validate_config(
        &self,
        config: &PowerManagement,
        driver: &str,
    ) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // 检查驱动支持
        if !self.is_driver_supported(driver) {
            errors.push(format!(
                "Power management configuration is not supported by driver '{}'. Supported drivers: {}",
                driver,
                self.supported_drivers.join(", ")
            ));
            return Err(errors);
        }

        // 基础验证
        match config.validate(Some(driver)) {
            Ok(_) => {}
            Err(warnings) => {
                // 这些是警告而不是错误
                for warning in warnings {
                    errors.push(format!("Warning: {}", warning));
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// 记录配置历史
    pub fn record_config(&mut self, config: PowerManagement) {
        self.config_history.push(config);
    }

    /// 获取配置历史摘要
    pub fn get_history_summary(&self) -> String {
        if self.config_history.is_empty() {
            return "No power management configuration history".to_string();
        }

        let mut summary = format!(
            "Power Management Configuration History ({} entries):\n",
            self.config_history.len()
        );

        for (i, config) in self.config_history.iter().enumerate() {
            summary.push_str(&format!("\nEntry {}:\n", i + 1));
            summary.push_str(&config.get_summary());
        }

        summary
    }

    /// 获取驱动支持信息
    pub fn get_driver_info(&self) -> String {
        let mut info = String::from("Power Management Driver Support:\n");
        info.push_str("==============================\n\n");

        for driver in &self.supported_drivers {
            match driver.as_str() {
                "qemu" => {
                    info.push_str("QEMU Driver:\n");
                    info.push_str("  - Full support for S3 (suspend-to-mem)\n");
                    info.push_str("  - Full support for S4 (suspend-to-disk)\n");
                    info.push_str("  - Controls BIOS ACPI advertisements\n");
                    info.push_str("  - Cannot prevent guest OS from suspending\n");
                }
                _ => {
                    info.push_str(&format!("{}: Not supported\n", driver));
                }
            }
            info.push('\n');
        }

        info.push_str("Unsupported Drivers:\n");
        info.push_str("  - All other drivers use hypervisor defaults\n");
        info.push_str("  - Configuration will be ignored\n");

        info
    }

    /// 获取推荐的状态转换表
    pub fn get_state_transition_table(&self) -> String {
        let mut table = String::from("ACPI Sleep State Transitions:\n");
        table.push_str("===============================\n\n");

        table.push_str("State  | Power Consumption | Resume Time | Data Location\n");
        table.push_str("------ | ----------------- | ----------- | -------------\n");
        table.push_str("S0     | Full power        | N/A         | N/A\n");
        table.push_str("S1     | Reduced           | Fast        | RAM\n");
        table.push_str("S2     | Reduced           | Fast        | RAM\n");
        table.push_str("S3     | Very low          | Moderate    | RAM\n");
        table.push_str("S4     | None              | Slow        | Disk\n");
        table.push_str("S5     | None              | Slow        | None (soft-off)\n\n");

        table.push_str("Key Points:\n");
        table.push_str("- S3: Suspend to RAM, data remains in memory\n");
        table.push_str("- S4: Hibernate, data saved to disk\n");
        table.push_str("- Guest OS can override BIOS advertisements\n");

        table
    }
}
