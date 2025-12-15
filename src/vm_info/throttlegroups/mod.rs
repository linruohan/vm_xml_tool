mod throttlegroup;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use throttlegroup::ThrottleGroup;

/// 限流组集合
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct ThrottleGroups {
    #[serde(rename = "throttlegroup", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<ThrottleGroup>>,
}

impl ThrottleGroups {
    /// 创建新的限流组集合
    pub fn new() -> Self {
        Self::default()
    }

    /// 添加限流组
    pub fn add_group(mut self, group: ThrottleGroup) -> Self {
        if self.groups.is_none() {
            self.groups = Some(Vec::new());
        }

        if let Some(ref mut groups) = self.groups {
            // 检查是否有重复的组名
            if !groups.iter().any(|g| g.name == group.name) {
                groups.push(group);
            }
        }

        self
    }

    /// 批量添加限流组
    pub fn with_groups(mut self, groups: Vec<ThrottleGroup>) -> Self {
        self.groups = Some(groups);
        self
    }

    /// 获取指定名称的限流组
    pub fn get_group(&self, name: &str) -> Option<&ThrottleGroup> {
        self.groups
            .as_ref()
            .and_then(|groups| groups.iter().find(|g| g.name == name))
    }

    /// 检查组是否存在
    pub fn has_group(&self, name: &str) -> bool {
        self.get_group(name).is_some()
    }

    /// 验证所有限流组
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if let Some(groups) = &self.groups {
            // 检查重复的组名
            let mut seen_names = HashMap::new();

            for (index, group) in groups.iter().enumerate() {
                // 验证单个组
                match group.validate() {
                    Ok(_) => {}
                    Err(mut group_errors) => {
                        errors.push(format!(
                            "Throttle group '{}' (index {}):",
                            group.name, index
                        ));
                        errors.append(&mut group_errors);
                    }
                }

                // 检查重复组名
                if let Some(prev_index) = seen_names.insert(&group.name, index) {
                    errors.push(format!(
                        "Duplicate throttle group name '{}' at positions {} and {}",
                        group.name,
                        prev_index + 1,
                        index + 1
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
        let mut summary = String::from("Throttle Groups Configuration:\n");

        if let Some(groups) = &self.groups {
            summary.push_str(&format!("  Total groups: {}\n", groups.len()));

            for (i, group) in groups.iter().enumerate() {
                summary.push_str(&format!("\n  Group {}:\n", i + 1));
                for line in group.get_summary().lines() {
                    summary.push_str(&format!("  {}\n", line));
                }

                // 添加建议的磁盘数量
                let suggested_disks = group.get_suggested_disk_count();
                summary.push_str(&format!("    Suggested max disks: {}\n", suggested_disks));
            }
        } else {
            summary.push_str("  No throttle groups configured\n");
        }

        summary
    }

    /// 获取所有组的总吞吐量限制
    pub fn get_total_throughput_capacity(&self) -> u64 {
        self.groups
            .as_ref()
            .map(|groups| {
                groups
                    .iter()
                    .filter_map(|g| g.get_total_throughput_limit())
                    .sum()
            })
            .unwrap_or(0)
    }

    /// 获取所有组的总 IOPS 容量
    pub fn get_total_iops_capacity(&self) -> u64 {
        self.groups
            .as_ref()
            .map(|groups| groups.iter().filter_map(|g| g.get_total_iops_limit()).sum())
            .unwrap_or(0)
    }

    /// 获取建议的配置（基于常见场景）
    pub fn suggested_config(scenario: &str) -> Option<Self> {
        match scenario.to_lowercase().as_str() {
            "development" => Some(
                Self::new().add_group(
                    ThrottleGroup::new("dev-default")
                        .with_total_bytes_per_sec(50_000_000) // 50 MB/s
                        .with_total_iops_per_sec(5_000), // 5k IOPS
                ),
            ),
            "production" => Some(
                Self::new()
                    .add_group(
                        ThrottleGroup::new("high-performance")
                            .with_total_bytes_per_sec(200_000_000) // 200 MB/s
                            .with_total_iops_per_sec(20_000), // 20k IOPS
                    )
                    .add_group(
                        ThrottleGroup::new("standard")
                            .with_total_bytes_per_sec(100_000_000) // 100 MB/s
                            .with_total_iops_per_sec(10_000), // 10k IOPS
                    )
                    .add_group(
                        ThrottleGroup::new("low-priority")
                            .with_total_bytes_per_sec(25_000_000) // 25 MB/s
                            .with_total_iops_per_sec(2_500), // 2.5k IOPS
                    ),
            ),
            "database" => Some(
                Self::new()
                    .add_group(
                        ThrottleGroup::new("db-data")
                            .with_total_iops_per_sec(50_000) // 高 IOPS
                            .with_write_bytes_per_sec(100_000_000), // 100 MB/s 写入
                    )
                    .add_group(
                        ThrottleGroup::new("db-log")
                            .with_write_bytes_per_sec(50_000_000) // 50 MB/s 写入
                            .with_write_iops_per_sec(10_000), // 10k 写入 IOPS
                    ),
            ),
            "storage" => Some(
                Self::new()
                    .add_group(
                        ThrottleGroup::new("fast-storage")
                            .with_total_bytes_per_sec(500_000_000) // 500 MB/s
                            .with_total_iops_per_sec(100_000), // 100k IOPS
                    )
                    .add_group(
                        ThrottleGroup::new("archive")
                            .with_total_bytes_per_sec(50_000_000) // 50 MB/s
                            .with_total_iops_per_sec(1_000), // 1k IOPS
                    ),
            ),
            _ => None,
        }
    }
}
