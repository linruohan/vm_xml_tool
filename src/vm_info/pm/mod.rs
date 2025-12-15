mod power_management_manager;
mod sleep_state;

use serde::{Deserialize, Serialize};
use sleep_state::{SleepState, SleepStateConfig};

/// 电源管理配置
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct PowerManagement {
    /// 挂起到内存配置
    #[serde(rename = "suspend-to-mem", skip_serializing_if = "Option::is_none")]
    pub suspend_to_mem: Option<SleepStateConfig>,

    /// 挂起到磁盘配置
    #[serde(rename = "suspend-to-disk", skip_serializing_if = "Option::is_none")]
    pub suspend_to_disk: Option<SleepStateConfig>,
}

impl PowerManagement {
    /// 创建新的电源管理配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置挂起到内存配置
    pub fn with_suspend_to_mem(mut self, enabled: String) -> Self {
        self.suspend_to_mem = Some(SleepStateConfig::new(SleepState::SuspendToMem, enabled));
        self
    }

    /// 设置挂起到磁盘配置
    pub fn with_suspend_to_disk(mut self, enabled: String) -> Self {
        self.suspend_to_disk = Some(SleepStateConfig::new(SleepState::SuspendToDisk, enabled));
        self
    }

    /// 启用所有睡眠状态
    pub fn enable_all() -> Self {
        Self::new()
            .with_suspend_to_mem("yes".to_string())
            .with_suspend_to_disk("yes".to_string())
    }

    /// 禁用所有睡眠状态
    pub fn disable_all() -> Self {
        Self::new()
            .with_suspend_to_mem("no".to_string())
            .with_suspend_to_disk("no".to_string())
    }

    /// 获取默认配置（基于典型设置）
    pub fn typical() -> Self {
        Self::new()
            .with_suspend_to_mem("yes".to_string()) // S3 通常启用
            .with_suspend_to_disk("no".to_string()) // S4 通常禁用
    }

    /// 获取指定的睡眠状态配置
    pub fn get_state_config(&self, state: SleepState) -> Option<&SleepStateConfig> {
        match state {
            SleepState::SuspendToMem => self.suspend_to_mem.as_ref(),
            SleepState::SuspendToDisk => self.suspend_to_disk.as_ref(),
        }
    }

    /// 检查睡眠状态是否启用
    pub fn is_state_enabled(&self, state: SleepState) -> bool {
        self.get_state_config(state)
            .map(|config| config.enabled == "yes")
            .unwrap_or(false)
    }

    /// 验证配置
    pub fn validate(&self, driver: Option<&str>) -> Result<(), Vec<String>> {
        let mut warnings = Vec::new();

        // 检查驱动支持
        if let Some(driver_name) = driver {
            let driver_lower = driver_name.to_lowercase();

            // 目前只有 QEMU 支持电源管理配置
            if driver_lower != "qemu" {
                warnings.push(format!(
                    "Power management configuration is only supported by QEMU driver, current driver: {}",
                    driver_name
                ));
            }
        }

        // 检查配置合理性
        if let Some(s3_config) = &self.suspend_to_mem
            && s3_config.enabled.parse().unwrap()
        {
            // S3 启用的常见警告
            warnings.push("S3 (suspend-to-mem) enabled: Guest OS may suspend to RAM".to_string());
        }

        if let Some(s4_config) = &self.suspend_to_disk
            && s4_config.enabled.parse().unwrap()
        {
            // S4 启用的警告
            warnings.push("S4 (suspend-to-disk) enabled: Guest OS may hibernate".to_string());
            warnings.push(
                "Note: S4 state cannot be prevented if guest OS chooses to circumvent BIOS"
                    .to_string(),
            );
        }

        // 检查是否有任何睡眠状态启用
        let any_enabled = self.is_state_enabled(SleepState::SuspendToMem)
            || self.is_state_enabled(SleepState::SuspendToDisk);

        if any_enabled {
            warnings.push(
                "Warning: Enabling sleep states may cause unexpected guest suspension".to_string(),
            );
        }

        if warnings.is_empty() {
            Ok(())
        } else {
            Err(warnings)
        }
    }

    /// 获取配置摘要
    pub fn get_summary(&self) -> String {
        let mut summary = String::from("Power Management Configuration:\n");

        summary.push_str("  ACPI Sleep States:\n");

        let states = vec![
            (SleepState::SuspendToMem, self.suspend_to_mem.as_ref()),
            (SleepState::SuspendToDisk, self.suspend_to_disk.as_ref()),
        ];

        for (state, config_opt) in states {
            summary.push_str(&format!("    {} ({}): ", state.acpi_state(), state));

            if let Some(config) = config_opt {
                summary.push_str(&format!(
                    "{}\n",
                    if config.enabled == "yes" {
                        "enabled"
                    } else {
                        "disabled"
                    }
                ));
            } else {
                summary.push_str("(use hypervisor default)\n");
            }
        }

        summary
    }

    /// 获取详细的配置信息
    pub fn get_detailed_info(&self) -> String {
        let mut info = String::new();

        info.push_str("Power Management Details:\n");
        info.push_str("=========================\n\n");

        info.push_str("ACPI Sleep States:\n");
        info.push_str("-----------------\n");

        let states_info = vec![
            (
                "S3 (suspend-to-mem)",
                "Suspend to RAM",
                "Low power state where system state is saved to RAM",
                self.suspend_to_mem.as_ref(),
            ),
            (
                "S4 (suspend-to-disk)",
                "Suspend to Disk (Hibernate)",
                "System state is saved to disk, allowing power to be completely removed",
                self.suspend_to_disk.as_ref(),
            ),
        ];

        for (acpi_state, name, description, config_opt) in states_info {
            info.push_str(&format!("\n{} - {}\n", acpi_state, name));
            info.push_str(&format!("  Description: {}\n", description));
            info.push_str("  Configuration: ");

            if let Some(config) = config_opt {
                info.push_str(&format!(
                    "{}\n",
                    if config.enabled == "yes" {
                        "enabled"
                    } else {
                        "disabled"
                    }
                ));
            } else {
                info.push_str("not configured (hypervisor default)\n");
            }
        }

        info.push_str("\nImportant Notes:\n");
        info.push_str("----------------\n");
        info.push_str("1. This setting only controls BIOS advertisements to the guest OS\n");
        info.push_str("2. Guest OS may circumvent these settings and suspend anyway\n");
        info.push_str("3. Currently only supported by QEMU driver\n");
        info.push_str("4. Enabling sleep states may cause unexpected VM suspension\n");

        info
    }

    /// 获取建议的配置（基于场景）
    pub fn suggested_config(scenario: &str) -> Option<Self> {
        match scenario.to_lowercase().as_str() {
            "typical" | "default" => Some(Self::typical()),
            "all-enabled" | "full-power" => Some(Self::enable_all()),
            "all-disabled" | "no-sleep" => Some(Self::disable_all()),
            "desktop" => Some(
                Self::new()
                    .with_suspend_to_mem("yes".to_string()) // 桌面系统通常支持睡眠
                    .with_suspend_to_disk("no".to_string()), // 但通常禁用休眠
            ),
            "server" => Some(
                Self::new()
                    .with_suspend_to_mem("no".to_string()) // 服务器通常禁用睡眠
                    .with_suspend_to_disk("no".to_string()), // 服务器禁用休眠
            ),
            "laptop" => Some(
                Self::new()
                    .with_suspend_to_mem("yes".to_string()) // 笔记本电脑通常支持睡眠
                    .with_suspend_to_disk("yes".to_string()), // 笔记本电脑支持休眠
            ),
            "cloud" | "virtualization" => Some(
                Self::new()
                    .with_suspend_to_mem("no".to_string()) // 云/Virtualization 环境禁用睡眠
                    .with_suspend_to_disk("no".to_string()),
            ),
            _ => None,
        }
    }
}
