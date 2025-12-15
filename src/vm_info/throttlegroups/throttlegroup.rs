use serde::{Deserialize, Serialize};

/// 限流组配置
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ThrottleGroup {
    /// 组名（必需）
    #[serde(rename = "group_name")]
    pub name: String,

    /// 总字节数/秒
    #[serde(rename = "total_bytes_sec", skip_serializing_if = "Option::is_none")]
    pub total_bytes_per_sec: Option<u64>,

    /// 读取字节数/秒
    #[serde(rename = "read_bytes_sec", skip_serializing_if = "Option::is_none")]
    pub read_bytes_per_sec: Option<u64>,

    /// 写入字节数/秒
    #[serde(rename = "write_bytes_sec", skip_serializing_if = "Option::is_none")]
    pub write_bytes_per_sec: Option<u64>,

    /// 总 IOPS
    #[serde(rename = "total_iops_sec", skip_serializing_if = "Option::is_none")]
    pub total_iops_per_sec: Option<u64>,

    /// 读取 IOPS
    #[serde(rename = "read_iops_sec", skip_serializing_if = "Option::is_none")]
    pub read_iops_per_sec: Option<u64>,

    /// 写入 IOPS
    #[serde(rename = "write_iops_sec", skip_serializing_if = "Option::is_none")]
    pub write_iops_per_sec: Option<u64>,

    /// 总字节数/秒最大值
    #[serde(
        rename = "total_bytes_sec_max",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_bytes_per_sec_max: Option<u64>,

    /// 读取字节数/秒最大值
    #[serde(rename = "read_bytes_sec_max", skip_serializing_if = "Option::is_none")]
    pub read_bytes_per_sec_max: Option<u64>,

    /// 写入字节数/秒最大值
    #[serde(
        rename = "write_bytes_sec_max",
        skip_serializing_if = "Option::is_none"
    )]
    pub write_bytes_per_sec_max: Option<u64>,

    /// 总 IOPS 最大值
    #[serde(rename = "total_iops_sec_max", skip_serializing_if = "Option::is_none")]
    pub total_iops_per_sec_max: Option<u64>,

    /// 读取 IOPS 最大值
    #[serde(rename = "read_iops_sec_max", skip_serializing_if = "Option::is_none")]
    pub read_iops_per_sec_max: Option<u64>,

    /// 写入 IOPS 最大值
    #[serde(rename = "write_iops_sec_max", skip_serializing_if = "Option::is_none")]
    pub write_iops_per_sec_max: Option<u64>,

    /// 总字节数/秒最大长度
    #[serde(
        rename = "total_bytes_sec_max_length",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_bytes_per_sec_max_length: Option<u32>,

    /// 读取字节数/秒最大长度
    #[serde(
        rename = "read_bytes_sec_max_length",
        skip_serializing_if = "Option::is_none"
    )]
    pub read_bytes_per_sec_max_length: Option<u32>,

    /// 写入字节数/秒最大长度
    #[serde(
        rename = "write_bytes_sec_max_length",
        skip_serializing_if = "Option::is_none"
    )]
    pub write_bytes_per_sec_max_length: Option<u32>,

    /// 总 IOPS 最大长度
    #[serde(
        rename = "total_iops_sec_max_length",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_iops_per_sec_max_length: Option<u32>,

    /// 读取 IOPS 最大长度
    #[serde(
        rename = "read_iops_sec_max_length",
        skip_serializing_if = "Option::is_none"
    )]
    pub read_iops_per_sec_max_length: Option<u32>,

    /// 写入 IOPS 最大长度
    #[serde(
        rename = "write_iops_sec_max_length",
        skip_serializing_if = "Option::is_none"
    )]
    pub write_iops_per_sec_max_length: Option<u32>,
}

impl ThrottleGroup {
    /// 创建新的限流组
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            total_bytes_per_sec: None,
            read_bytes_per_sec: None,
            write_bytes_per_sec: None,
            total_iops_per_sec: None,
            read_iops_per_sec: None,
            write_iops_per_sec: None,
            total_bytes_per_sec_max: None,
            read_bytes_per_sec_max: None,
            write_bytes_per_sec_max: None,
            total_iops_per_sec_max: None,
            read_iops_per_sec_max: None,
            write_iops_per_sec_max: None,
            total_bytes_per_sec_max_length: None,
            read_bytes_per_sec_max_length: None,
            write_bytes_per_sec_max_length: None,
            total_iops_per_sec_max_length: None,
            read_iops_per_sec_max_length: None,
            write_iops_per_sec_max_length: None,
        }
    }

    /// 设置总字节数/秒限制
    pub fn with_total_bytes_per_sec(mut self, bytes_per_sec: u64) -> Self {
        self.total_bytes_per_sec = Some(bytes_per_sec);
        self
    }

    /// 设置读取字节数/秒限制
    pub fn with_read_bytes_per_sec(mut self, bytes_per_sec: u64) -> Self {
        self.read_bytes_per_sec = Some(bytes_per_sec);
        self
    }

    /// 设置写入字节数/秒限制
    pub fn with_write_bytes_per_sec(mut self, bytes_per_sec: u64) -> Self {
        self.write_bytes_per_sec = Some(bytes_per_sec);
        self
    }

    /// 设置总 IOPS 限制
    pub fn with_total_iops_per_sec(mut self, iops_per_sec: u64) -> Self {
        self.total_iops_per_sec = Some(iops_per_sec);
        self
    }

    /// 设置读取 IOPS 限制
    pub fn with_read_iops_per_sec(mut self, iops_per_sec: u64) -> Self {
        self.read_iops_per_sec = Some(iops_per_sec);
        self
    }

    /// 设置写入 IOPS 限制
    pub fn with_write_iops_per_sec(mut self, iops_per_sec: u64) -> Self {
        self.write_iops_per_sec = Some(iops_per_sec);
        self
    }

    /// 设置双向字节数/秒限制
    pub fn with_bytes_per_sec(self, bytes_per_sec: u64) -> Self {
        self.with_read_bytes_per_sec(bytes_per_sec)
            .with_write_bytes_per_sec(bytes_per_sec)
    }

    /// 设置双向 IOPS 限制
    pub fn with_iops_per_sec(self, iops_per_sec: u64) -> Self {
        self.with_read_iops_per_sec(iops_per_sec)
            .with_write_iops_per_sec(iops_per_sec)
    }

    /// 验证限流组配置
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // 检查组名
        if self.name.is_empty() {
            errors.push("Throttle group name cannot be empty".to_string());
        }

        if self.name.len() > 64 {
            errors.push(format!(
                "Throttle group name '{}' is too long (max 64 characters)",
                self.name
            ));
        }

        // 检查无效字符
        if self
            .name
            .contains(|c: char| c.is_whitespace() || c.is_control())
        {
            errors.push(format!(
                "Throttle group name '{}' contains invalid characters",
                self.name
            ));
        }

        // 验证限流值
        self.validate_throttle_value("total_bytes_per_sec", self.total_bytes_per_sec, &mut errors);
        self.validate_throttle_value("read_bytes_per_sec", self.read_bytes_per_sec, &mut errors);
        self.validate_throttle_value("write_bytes_per_sec", self.write_bytes_per_sec, &mut errors);
        self.validate_throttle_value("total_iops_per_sec", self.total_iops_per_sec, &mut errors);
        self.validate_throttle_value("read_iops_per_sec", self.read_iops_per_sec, &mut errors);
        self.validate_throttle_value("write_iops_per_sec", self.write_iops_per_sec, &mut errors);

        // 验证最大值
        self.validate_max_value(
            "total_bytes_per_sec",
            self.total_bytes_per_sec,
            "total_bytes_per_sec_max",
            self.total_bytes_per_sec_max,
            &mut errors,
        );
        self.validate_max_value(
            "read_bytes_per_sec",
            self.read_bytes_per_sec,
            "read_bytes_per_sec_max",
            self.read_bytes_per_sec_max,
            &mut errors,
        );
        self.validate_max_value(
            "write_bytes_per_sec",
            self.write_bytes_per_sec,
            "write_bytes_per_sec_max",
            self.write_bytes_per_sec_max,
            &mut errors,
        );
        self.validate_max_value(
            "total_iops_per_sec",
            self.total_iops_per_sec,
            "total_iops_per_sec_max",
            self.total_iops_per_sec_max,
            &mut errors,
        );
        self.validate_max_value(
            "read_iops_per_sec",
            self.read_iops_per_sec,
            "read_iops_per_sec_max",
            self.read_iops_per_sec_max,
            &mut errors,
        );
        self.validate_max_value(
            "write_iops_per_sec",
            self.write_iops_per_sec,
            "write_iops_per_sec_max",
            self.write_iops_per_sec_max,
            &mut errors,
        );

        // 检查是否有至少一个限制设置
        if !self.has_any_limit_set() {
            errors.push(format!(
                "Throttle group '{}' has no limits configured",
                self.name
            ));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn validate_throttle_value(&self, name: &str, value: Option<u64>, errors: &mut Vec<String>) {
        if let Some(val) = value {
            if val == 0 {
                errors.push(format!("{} cannot be 0", name));
            }

            // 检查是否过大（1 PB/s）
            if val > 1_000_000_000_000_000 {
                errors.push(format!("{} ({}) is unrealistically high", name, val));
            }
        }
    }

    fn validate_max_value(
        &self,
        base_name: &str,
        base_value: Option<u64>,
        max_name: &str,
        max_value: Option<u64>,
        errors: &mut Vec<String>,
    ) {
        if let (Some(base), Some(max)) = (base_value, max_value)
            && max < base {
                errors.push(format!(
                    "{} ({}) cannot be less than {} ({})",
                    max_name, max, base_name, base
                ));
            }
    }

    fn has_any_limit_set(&self) -> bool {
        self.total_bytes_per_sec.is_some()
            || self.read_bytes_per_sec.is_some()
            || self.write_bytes_per_sec.is_some()
            || self.total_iops_per_sec.is_some()
            || self.read_iops_per_sec.is_some()
            || self.write_iops_per_sec.is_some()
    }

    /// 获取配置摘要
    pub fn get_summary(&self) -> String {
        let mut summary = format!("Throttle Group: '{}'\n", self.name);

        // 吞吐量限制
        let throughput = self.get_throughput_summary();
        if !throughput.is_empty() {
            summary.push_str("  Throughput Limits:\n");
            summary.push_str(&throughput);
        }

        // IOPS 限制
        let iops = self.get_iops_summary();
        if !iops.is_empty() {
            summary.push_str("  IOPS Limits:\n");
            summary.push_str(&iops);
        }

        // 最大限制
        let max_limits = self.get_max_limits_summary();
        if !max_limits.is_empty() {
            summary.push_str("  Maximum Limits:\n");
            summary.push_str(&max_limits);
        }

        // 时间长度限制
        let length_limits = self.get_length_limits_summary();
        if !length_limits.is_empty() {
            summary.push_str("  Time Length Limits:\n");
            summary.push_str(&length_limits);
        }

        summary
    }

    fn get_throughput_summary(&self) -> String {
        let mut summary = String::new();

        if let Some(total) = self.total_bytes_per_sec {
            summary.push_str(&format!(
                "    Total: {:.2} MB/s\n",
                total as f64 / 1_000_000.0
            ));
        }
        if let Some(read) = self.read_bytes_per_sec {
            summary.push_str(&format!(
                "    Read: {:.2} MB/s\n",
                read as f64 / 1_000_000.0
            ));
        }
        if let Some(write) = self.write_bytes_per_sec {
            summary.push_str(&format!(
                "    Write: {:.2} MB/s\n",
                write as f64 / 1_000_000.0
            ));
        }

        summary
    }

    fn get_iops_summary(&self) -> String {
        let mut summary = String::new();

        if let Some(total) = self.total_iops_per_sec {
            summary.push_str(&format!("    Total: {} IOPS\n", total));
        }
        if let Some(read) = self.read_iops_per_sec {
            summary.push_str(&format!("    Read: {} IOPS\n", read));
        }
        if let Some(write) = self.write_iops_per_sec {
            summary.push_str(&format!("    Write: {} IOPS\n", write));
        }

        summary
    }

    fn get_max_limits_summary(&self) -> String {
        let mut summary = String::new();

        let max_checks = vec![
            ("Total Bytes", self.total_bytes_per_sec_max),
            ("Read Bytes", self.read_bytes_per_sec_max),
            ("Write Bytes", self.write_bytes_per_sec_max),
            ("Total IOPS", self.total_iops_per_sec_max),
            ("Read IOPS", self.read_iops_per_sec_max),
            ("Write IOPS", self.write_iops_per_sec_max),
        ];

        for (name, value) in max_checks {
            if let Some(val) = value {
                summary.push_str(&format!("    {}: {}\n", name, val));
            }
        }

        summary
    }

    fn get_length_limits_summary(&self) -> String {
        let mut summary = String::new();

        let length_checks = vec![
            ("Total Bytes", self.total_bytes_per_sec_max_length),
            ("Read Bytes", self.read_bytes_per_sec_max_length),
            ("Write Bytes", self.write_bytes_per_sec_max_length),
            ("Total IOPS", self.total_iops_per_sec_max_length),
            ("Read IOPS", self.read_iops_per_sec_max_length),
            ("Write IOPS", self.write_iops_per_sec_max_length),
        ];

        for (name, value) in length_checks {
            if let Some(val) = value {
                summary.push_str(&format!("    {}: {} seconds\n", name, val));
            }
        }

        summary
    }

    /// 计算总吞吐量限制
    pub fn get_total_throughput_limit(&self) -> Option<u64> {
        self.total_bytes_per_sec
    }

    /// 计算总 IOPS 限制
    pub fn get_total_iops_limit(&self) -> Option<u64> {
        self.total_iops_per_sec
    }

    /// 获取该组支持的磁盘数量建议
    pub fn get_suggested_disk_count(&self) -> usize {
        let throughput = self.get_total_throughput_limit().unwrap_or(100_000_000); // 默认 100 MB/s

        let iops = self.get_total_iops_limit().unwrap_or(10_000); // 默认 10k IOPS

        // 简单的启发式算法
        let by_throughput = (throughput / 25_000_000).max(1) as usize; // 每个磁盘 25 MB/s
        let by_iops = (iops / 2_500).max(1) as usize; // 每个磁盘 2.5k IOPS

        by_throughput.min(by_iops).min(16) // 最多 16 个磁盘
    }
}
