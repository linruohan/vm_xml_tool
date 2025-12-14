use crate::vm_info::resource::ResourceConfig;
use crate::vm_info::resource::fibrechannel::FibreChannelConfig;
use crate::vm_info::resource::partition::PartitionPath;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// 资源分区管理器
#[derive(Debug, Default)]
pub struct ResourceManager {
    /// 已知的分区
    known_partitions: Vec<PartitionPath>,
    /// 默认分区
    default_partition: PartitionPath,
    /// 支持的驱动
    supported_drivers: Vec<String>,
}

impl ResourceManager {
    /// 创建新的资源管理器
    pub fn new() -> Self {
        Self {
            known_partitions: vec![PartitionPath::new("/")],
            default_partition: PartitionPath::new("/"),
            supported_drivers: vec!["qemu".to_string(), "lxc".to_string()],
        }
    }

    /// 设置默认分区
    pub fn with_default_partition<P: AsRef<Path>>(mut self, path: P) -> Self {
        let partition = PartitionPath::new(path).normalize();
        self.default_partition = partition.clone();

        // 确保默认分区在已知分区列表中
        if !self.known_partitions.contains(&partition) {
            self.known_partitions.push(partition);
        }

        self
    }

    /// 添加已知分区
    pub fn add_partition<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Vec<String>> {
        let partition = PartitionPath::new(path).normalize();

        // 验证分区
        match partition.validate() {
            Ok(_) => {
                if !self.known_partitions.contains(&partition) {
                    self.known_partitions.push(partition);
                }
                Ok(())
            }
            Err(errors) => Err(errors),
        }
    }

    /// 检查分区是否存在
    pub fn partition_exists<P: AsRef<Path>>(&self, path: P) -> bool {
        let partition = PartitionPath::new(path).normalize();

        // 检查是否为已知分区
        self.known_partitions.contains(&partition)
    }

    /// 验证域的资源配置
    pub fn validate_domain_config(
        &self,
        resource: &ResourceConfig,
        driver: &str,
    ) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // 检查驱动支持
        if !self.supported_drivers.iter().any(|d| d == driver) {
            errors.push(format!(
                "Driver '{}' does not support resource partitioning. Supported drivers: {}",
                driver,
                self.supported_drivers.join(", ")
            ));
        }

        // 验证分区配置
        if let Some(partition_config) = &resource.partition {
            let partition_path = &partition_config.path;

            // 验证分区路径格式
            match partition_path.validate() {
                Ok(_) => {}
                Err(mut partition_errors) => {
                    errors.append(&mut partition_errors);
                }
            }

            // 检查分区是否存在（默认分区总是存在）
            if partition_path.normalize() != self.default_partition
                && !self.partition_exists(partition_path.as_path())
            {
                errors.push(format!(
                    "Partition '{}' does not exist. It must be created before starting the domain.",
                    partition_path.as_str()
                ));
            }

            // 对于嵌套分区，检查父分区是否存在
            if partition_config.is_nested()
                && let Some(parent) = partition_path.get_parent()
                && !self.partition_exists(parent.as_path())
            {
                errors.push(format!(
                    "Parent partition '{}' does not exist for nested partition '{}'",
                    parent.as_str(),
                    partition_path.as_str()
                ));
            }
        }

        // 验证 Fibre Channel 配置
        if let Some(fc_config) = &resource.fibre_channel {
            match fc_config.validate() {
                Ok(_) => {}
                Err(mut fc_errors) => {
                    errors.append(&mut fc_errors);
                }
            }

            // 检查 Fibre Channel 系统要求
            match FibreChannelConfig::check_requirements() {
                Ok(_) => {}
                Err(req_errors) => {
                    // 添加为警告而不是错误
                    for error in req_errors {
                        errors.push(format!("Warning: {}", error));
                    }
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// 获取分区层次结构
    pub fn get_partition_hierarchy(&self) -> Vec<PartitionPath> {
        let mut hierarchy = self.known_partitions.clone();
        hierarchy.sort_by_key(|a| a.get_depth());
        hierarchy
    }

    /// 获取与cgroups的映射关系
    pub fn get_cgroup_mapping(&self, partition: &PartitionPath) -> HashMap<String, PathBuf> {
        let mut mappings = HashMap::new();

        // 常见的cgroup控制器
        let controllers = vec![
            "cpu",
            "cpuacct",
            "cpuset",
            "memory",
            "blkio",
            "devices",
            "freezer",
            "net_cls",
            "net_prio",
            "perf_event",
            "pids",
            "rdma",
        ];

        let normalized = partition.normalize();

        for controller in controllers {
            let mut cgroup_path = PathBuf::from("/sys/fs/cgroup");
            cgroup_path.push(controller);

            if !normalized.is_root() {
                // 移除开头的斜杠
                let relative_path = normalized.as_str().trim_start_matches('/');
                cgroup_path.push(relative_path);
            }

            mappings.insert(controller.to_string(), cgroup_path);
        }

        mappings
    }

    /// 获取摘要信息
    pub fn get_summary(&self) -> String {
        let mut summary = String::from("Resource Manager:\n");
        summary.push_str(&format!(
            "  Default partition: {}\n",
            self.default_partition.as_str()
        ));
        summary.push_str(&format!(
            "  Known partitions: {}\n",
            self.known_partitions.len()
        ));
        summary.push_str(&format!(
            "  Supported drivers: {}\n",
            self.supported_drivers.join(", ")
        ));

        if !self.known_partitions.is_empty() {
            summary.push_str("\n  Partition hierarchy:\n");
            for partition in self.get_partition_hierarchy() {
                let indent = "    ".repeat(partition.get_depth());
                summary.push_str(&format!(
                    "{}{} (depth: {})\n",
                    indent,
                    partition.as_str(),
                    partition.get_depth()
                ));
            }
        }

        summary
    }
}
