use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
mod device;
use device::IoDevice;

/// I/O 调优配置
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct BlkioTune {
    /// 整体 I/O 权重 [10, 1000] 或 [100, 1000]
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<u32>,

    /// 设备特定的调优配置
    #[serde(rename = "device", skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<IoDevice>>,
}

impl BlkioTune {
    /// 创建新的 I/O 调优配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置整体权重
    pub fn with_weight(mut self, weight: u32) -> Self {
        self.weight = Some(weight);
        self
    }

    /// 添加设备配置
    pub fn add_device(mut self, device: IoDevice) -> Self {
        if self.devices.is_none() {
            self.devices = Some(Vec::new());
        }

        if let Some(ref mut devices) = self.devices {
            // 检查是否已存在相同路径的设备
            if let Some(index) = devices.iter().position(|d| d.path == device.path) {
                devices[index] = device; // 替换现有配置
            } else {
                devices.push(device);
            }
        }

        self
    }

    /// 批量添加设备配置
    pub fn with_devices(mut self, devices: Vec<IoDevice>) -> Self {
        self.devices = Some(devices);
        self
    }

    /// 获取指定路径的设备配置
    pub fn get_device<P: AsRef<Path>>(&self, path: P) -> Option<&IoDevice> {
        self.devices
            .as_ref()
            .and_then(|devices| devices.iter().find(|d| d.path == path.as_ref()))
    }

    /// 验证配置的有效性
    pub fn validate(&self, kernel_version: Option<KernelVersion>) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // 确定权重范围
        let (min_weight, max_weight) = match kernel_version {
            Some(KernelVersion::V2_6_39Plus) => (10, 1000),
            _ => (100, 1000),
        };

        // 验证整体权重
        if let Some(weight) = self.weight
            && (weight < min_weight || weight > max_weight)
        {
            errors.push(format!(
                "整体权重必须在 [{}, {}] 范围内，当前值: {}",
                min_weight, max_weight, weight
            ));
        }

        // 验证设备配置
        if let Some(devices) = &self.devices {
            for (index, device) in devices.iter().enumerate() {
                let mut device_errors = Vec::new();

                // 验证路径
                if device.path.as_os_str().is_empty() {
                    device_errors.push("设备路径不能为空".to_string());
                } else if !device.path.is_absolute() {
                    device_errors.push(format!("设备路径必须是绝对路径: {:?}", device.path));
                }

                // 验证权重
                if let Some(weight) = device.weight
                    && (weight < min_weight || weight > max_weight)
                {
                    device_errors.push(format!(
                        "设备权重必须在 [{}, {}] 范围内，当前值: {}",
                        min_weight, max_weight, weight
                    ));
                }

                // 验证吞吐量限制（可选）
                if let Some(bytes) = device.read_bytes_per_sec
                    && bytes == 0
                {
                    device_errors.push("读取吞吐量限制不能为 0".to_string());
                }

                if let Some(bytes) = device.write_bytes_per_sec
                    && bytes == 0
                {
                    device_errors.push("写入吞吐量限制不能为 0".to_string());
                }

                // 验证 IOPS 限制（可选）
                if let Some(iops) = device.read_iops_per_sec
                    && iops == 0
                {
                    device_errors.push("读取 IOPS 限制不能为 0".to_string());
                }

                if let Some(iops) = device.write_iops_per_sec
                    && iops == 0
                {
                    device_errors.push("写入 IOPS 限制不能为 0".to_string());
                }

                if !device_errors.is_empty() {
                    errors.push(format!("设备 {} (路径: {:?}):", index, device.path));
                    for err in device_errors {
                        errors.push(format!("  - {}", err));
                    }
                }
            }

            // 检查重复路径
            let mut seen_paths = HashMap::new();
            for (index, device) in devices.iter().enumerate() {
                if let Some(prev_index) = seen_paths.insert(device.path.clone(), index) {
                    errors.push(format!(
                        "重复的设备路径: {:?} (位置 {} 和 {})",
                        device.path, prev_index, index
                    ));
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// 获取配置摘要
    pub fn get_summary(&self) -> String {
        let mut summary = String::from("Block I/O Tuning Configuration:\n");

        if let Some(weight) = self.weight {
            summary.push_str(&format!("  Overall weight: {}\n", weight));
        } else {
            summary.push_str("  Overall weight: Not set (using OS defaults)\n");
        }

        if let Some(devices) = &self.devices {
            summary.push_str(&format!("  Devices configured: {}\n", devices.len()));

            for device in devices {
                summary.push_str(&format!("\n  Device: {:?}\n", device.path));

                if let Some(weight) = device.weight {
                    summary.push_str(&format!("    Weight: {}\n", weight));
                }

                if let Some(bytes) = device.read_bytes_per_sec {
                    summary.push_str(&format!("    Read throughput: {} B/s\n", bytes));
                }

                if let Some(bytes) = device.write_bytes_per_sec {
                    summary.push_str(&format!("    Write throughput: {} B/s\n", bytes));
                }

                if let Some(iops) = device.read_iops_per_sec {
                    summary.push_str(&format!("    Read IOPS: {}/s\n", iops));
                }

                if let Some(iops) = device.write_iops_per_sec {
                    summary.push_str(&format!("    Write IOPS: {}/s\n", iops));
                }
            }
        } else {
            summary.push_str("  No device-specific tuning configured\n");
        }

        summary
    }

    /// 计算总吞吐量限制
    pub fn calculate_total_throughput(&self) -> (u64, u64) {
        let mut total_read = 0u64;
        let mut total_write = 0u64;

        if let Some(devices) = &self.devices {
            for device in devices {
                total_read += device.read_bytes_per_sec.unwrap_or(0);
                total_write += device.write_bytes_per_sec.unwrap_or(0);
            }
        }

        (total_read, total_write)
    }

    /// 计算总 IOPS 限制
    pub fn calculate_total_iops(&self) -> (u64, u64) {
        let mut total_read_iops = 0u64;
        let mut total_write_iops = 0u64;

        if let Some(devices) = &self.devices {
            for device in devices {
                total_read_iops += device.read_iops_per_sec.unwrap_or(0);
                total_write_iops += device.write_iops_per_sec.unwrap_or(0);
            }
        }

        (total_read_iops, total_write_iops)
    }
}

/// 内核版本枚举
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum KernelVersion {
    Pre2_6_39,
    #[default]
    V2_6_39Plus,
}

impl KernelVersion {
    /// 从字符串解析内核版本
    pub fn from_str(version: &str) -> Option<Self> {
        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() >= 3 {
            let major = parts[0].parse::<u32>().ok()?;
            let minor = parts[1].parse::<u32>().ok()?;
            let patch = parts[2].parse::<u32>().ok()?;

            if major > 2 || (major == 2 && minor > 6) || (major == 2 && minor == 6 && patch >= 39) {
                Some(KernelVersion::V2_6_39Plus)
            } else {
                Some(KernelVersion::Pre2_6_39)
            }
        } else {
            None
        }
    }
}
