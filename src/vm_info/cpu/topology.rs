use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// CPU 拓扑配置
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct CpuTopology {
    /// CPU 插槽数（必须）
    #[serde(rename = "@sockets")]
    pub sockets: u32,

    /// 每个插槽的晶片数（可选，默认为1）
    #[serde(rename = "@dies", skip_serializing_if = "Option::is_none")]
    pub dies: Option<u32>,

    /// 每个晶片的集群数（可选，默认为1）
    #[serde(rename = "@clusters", skip_serializing_if = "Option::is_none")]
    pub clusters: Option<u32>,

    /// 每个集群的核心数（必须）
    #[serde(rename = "@cores")]
    pub cores: u32,

    /// 每个核心的线程数（必须）
    #[serde(rename = "@threads")]
    pub threads: u32,
}

impl CpuTopology {
    pub fn new(sockets: u32, cores: u32, threads: u32) -> Self {
        Self {
            sockets,
            dies: None,
            clusters: None,
            cores,
            threads,
        }
    }

    pub fn with_dies(mut self, dies: u32) -> Self {
        self.dies = Some(dies);
        self
    }

    pub fn with_clusters(mut self, clusters: u32) -> Self {
        self.clusters = Some(clusters);
        self
    }

    /// 计算总 vCPU 数量
    pub fn total_vcpus(&self) -> u32 {
        let dies = self.dies.unwrap_or(1);
        let clusters = self.clusters.unwrap_or(1);
        self.sockets * dies * clusters * self.cores * self.threads
    }

    /// 验证拓扑配置
    pub fn validate(&self, max_vcpus: Option<u32>) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // 检查基本参数
        if self.sockets == 0 {
            errors.push("sockets must be greater than 0".to_string());
        }
        if self.cores == 0 {
            errors.push("cores must be greater than 0".to_string());
        }
        if self.threads == 0 {
            errors.push("threads must be greater than 0".to_string());
        }

        if let Some(dies) = self.dies
            && dies == 0 {
                errors.push("dies must be greater than 0 if specified".to_string());
            }

        if let Some(clusters) = self.clusters
            && clusters == 0 {
                errors.push("clusters must be greater than 0 if specified".to_string());
            }

        // 检查 vCPU 数量
        let total_vcpus = self.total_vcpus();
        if total_vcpus == 0 {
            errors.push("total vCPUs cannot be 0".to_string());
        }

        // 检查与域配置的一致性
        if let Some(max_vcpus) = max_vcpus
            && total_vcpus > max_vcpus {
                errors.push(format!(
                    "CPU topology requires {} vCPUs but domain only allows {}",
                    total_vcpus, max_vcpus
                ));
            }

        // 检查常见限制
        if self.threads > 8 {
            errors.push(format!(
                "Unusually high thread count ({}). Most CPUs support up to 2 threads per core",
                self.threads
            ));
        }

        if self.cores > 128 {
            errors.push(format!(
                "Unusually high core count ({}). Verify system support",
                self.cores
            ));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// 获取拓扑的层级结构
    pub fn get_hierarchy(&self) -> HashMap<String, u32> {
        let mut hierarchy = HashMap::new();
        hierarchy.insert("sockets".to_string(), self.sockets);
        hierarchy.insert("dies".to_string(), self.dies.unwrap_or(1));
        hierarchy.insert("clusters".to_string(), self.clusters.unwrap_or(1));
        hierarchy.insert("cores".to_string(), self.cores);
        hierarchy.insert("threads".to_string(), self.threads);
        hierarchy
    }

    /// 获取 NUMA 节点建议（简化的）
    pub fn get_numa_suggestions(&self) -> Vec<u32> {
        let total_vcpus = self.total_vcpus();

        // 简单的 NUMA 节点建议规则
        if total_vcpus <= 4 {
            vec![1] // 小系统：单节点
        } else if total_vcpus <= 16 {
            vec![1, 2] // 中等系统：1-2个节点
        } else if total_vcpus <= 64 {
            vec![2, 4] // 大系统：2-4个节点
        } else {
            vec![4, 8, 16] // 大型系统：多个节点
        }
    }
}
