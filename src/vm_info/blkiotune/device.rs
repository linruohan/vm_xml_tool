use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// I/O 设备配置
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct IoDevice {
    /// 设备路径（绝对路径）
    #[serde(rename = "path")]
    pub path: PathBuf,

    /// 设备权重 [10, 1000] 或 [100, 1000]（取决于内核版本）
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<u32>,

    /// 读取吞吐量限制（字节/秒）
    #[serde(rename = "read_bytes_sec", skip_serializing_if = "Option::is_none")]
    pub read_bytes_per_sec: Option<u64>,

    /// 写入吞吐量限制（字节/秒）
    #[serde(rename = "write_bytes_sec", skip_serializing_if = "Option::is_none")]
    pub write_bytes_per_sec: Option<u64>,

    /// 读取 IOPS 限制
    #[serde(rename = "read_iops_sec", skip_serializing_if = "Option::is_none")]
    pub read_iops_per_sec: Option<u64>,

    /// 写入 IOPS 限制
    #[serde(rename = "write_iops_sec", skip_serializing_if = "Option::is_none")]
    pub write_iops_per_sec: Option<u64>,
}

impl IoDevice {
    /// 创建新的设备配置
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            weight: None,
            read_bytes_per_sec: None,
            write_bytes_per_sec: None,
            read_iops_per_sec: None,
            write_iops_per_sec: None,
        }
    }

    /// 设置权重
    pub fn with_weight(mut self, weight: u32) -> Self {
        self.weight = Some(weight);
        self
    }

    /// 设置读取吞吐量限制
    pub fn with_read_bytes_per_sec(mut self, bytes_per_sec: u64) -> Self {
        self.read_bytes_per_sec = Some(bytes_per_sec);
        self
    }

    /// 设置写入吞吐量限制
    pub fn with_write_bytes_per_sec(mut self, bytes_per_sec: u64) -> Self {
        self.write_bytes_per_sec = Some(bytes_per_sec);
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

    /// 设置双向吞吐量限制
    pub fn with_bytes_per_sec(mut self, bytes_per_sec: u64) -> Self {
        self.read_bytes_per_sec = Some(bytes_per_sec);
        self.write_bytes_per_sec = Some(bytes_per_sec);
        self
    }

    /// 设置双向 IOPS 限制
    pub fn with_iops_per_sec(mut self, iops_per_sec: u64) -> Self {
        self.read_iops_per_sec = Some(iops_per_sec);
        self.write_iops_per_sec = Some(iops_per_sec);
        self
    }
}
