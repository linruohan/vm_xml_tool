use serde::{Deserialize, Serialize};

mod partition;
mod fibrechannel;
use fibrechannel::FibreChannelConfig;
use partition::PartitionConfig;

/// 资源分区配置
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct ResourceConfig {
    /// 分区路径
    #[serde(rename = "partition", skip_serializing_if = "Option::is_none")]
    pub partition: Option<PartitionConfig>,

    /// Fibre Channel VMID 配置
    #[serde(rename = "fibrechannel", skip_serializing_if = "Option::is_none")]
    pub fibre_channel: Option<FibreChannelConfig>,
}

impl ResourceConfig {
    /// 创建新的资源配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置分区配置
    pub fn with_partition(mut self, partition: PartitionConfig) -> Self {
        self.partition = Some(partition);
        self
    }

    /// 设置 Fibre Channel 配置
    pub fn with_fibre_channel(mut self, fc_config: FibreChannelConfig) -> Self {
        self.fibre_channel = Some(fc_config);
        self
    }

    /// 验证配置
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // 验证分区配置
        if let Some(partition) = &self.partition {
            match partition.validate() {
                Ok(_) => {}
                Err(mut partition_errors) => {
                    errors.append(&mut partition_errors);
                }
            }
        }

        // 验证 Fibre Channel 配置
        if let Some(fc_config) = &self.fibre_channel {
            match fc_config.validate() {
                Ok(_) => {}
                Err(mut fc_errors) => {
                    errors.append(&mut fc_errors);
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
        let mut summary = String::from("Resource Configuration:\n");

        if let Some(partition) = &self.partition {
            summary.push_str(&partition.get_summary());
        } else {
            summary.push_str("  Partition: Not configured (using default)\n");
        }

        if let Some(fc_config) = &self.fibre_channel {
            summary.push_str(&fc_config.get_summary());
        } else {
            summary.push_str("  Fibre Channel: Not configured\n");
        }

        summary
    }
}