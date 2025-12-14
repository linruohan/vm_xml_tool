use serde::{Deserialize, Serialize};

/// Fibre Channel VMID 配置
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct FibreChannelConfig {
    /// 应用程序ID（VMID），最大128字节
    #[serde(rename = "@appid")]
    pub app_id: String,
}

impl FibreChannelConfig {
    /// 创建新的 Fibre Channel 配置
    pub fn new(app_id: &str) -> Self {
        Self {
            app_id: app_id.to_string(),
        }
    }

    /// 验证配置
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // 检查 app_id 长度
        if self.app_id.is_empty() {
            errors.push("Fibre Channel appid cannot be empty".to_string());
        } else if self.app_id.len() > 128 {
            errors.push(format!(
                "Fibre Channel appid too long ({} bytes, max 128)",
                self.app_id.len()
            ));
        }

        // 检查无效字符
        if self.app_id.contains('\0') {
            errors.push("Fibre Channel appid contains null character".to_string());
        }

        // 检查是否包含空格或控制字符
        if self.app_id.chars().any(|c| c.is_whitespace() || c.is_control()) {
            errors.push("Fibre Channel appid contains whitespace or control characters".to_string());
        }

        // 检查是否为有效的 ASCII 字符串（可选，取决于内核要求）
        if !self.app_id.is_ascii() {
            errors.push("Fibre Channel appid should be ASCII string".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// 获取摘要信息
    pub fn get_summary(&self) -> String {
        format!(
            "  Fibre Channel VMID:\n    App ID: {}\n    Length: {} bytes\n",
            self.app_id,
            self.app_id.len()
        )
    }

    /// 检查是否支持 Fibre Channel 功能
    pub fn check_requirements() -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // 这些检查在实际系统中可能需要系统调用
        // 这里只是示例验证

        errors.push("Note: Fibre Channel requires:".to_string());
        errors.push("  - Fibre Channel capable hardware".to_string());
        errors.push("  - Kernel with CONFIG_BLK_CGROUP_FC_APPID option".to_string());
        errors.push("  - nvme_fc kernel module loaded".to_string());

        if errors.len() > 1 {
            Err(errors)
        } else {
            Ok(())
        }
    }
}