mod cache;
mod cpu_match;
mod cpu_mode;
mod feature;
mod maxphysaddr;
mod model;
mod numa;
mod topology;

use crate::vm_info::cpu::maxphysaddr::MaxPhysAddr;
use cache::*;
use cpu_match::*;
use cpu_mode::*;
use feature::*;
use model::CpuModel;
pub use numa::NumaTopology;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use topology::CpuTopology;

/// CPU 检查模式
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum CpuCheck {
    None,
    Partial,
    #[default]
    Full,
}

/// CPU 配置
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct CpuConfig {
    /// 匹配模式
    #[serde(rename = "@match", skip_serializing_if = "Option::is_none")]
    pub match_mode: Option<CpuMatch>,

    /// 检查模式
    #[serde(rename = "@check", skip_serializing_if = "Option::is_none")]
    pub check: Option<CpuCheck>,

    /// CPU 模式
    #[serde(rename = "@mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<CpuMode>,

    /// 是否可迁移
    #[serde(rename = "@migratable", skip_serializing_if = "Option::is_none")]
    pub migratable: Option<bool>,

    /// 废弃特性处理（S390）
    #[serde(
        rename = "@deprecated_features",
        skip_serializing_if = "Option::is_none"
    )]
    pub deprecated_features: Option<String>, // "on" or "off"

    /// CPU 模型
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<CpuModel>,

    /// 供应商
    #[serde(rename = "vendor", skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,

    /// 拓扑配置
    #[serde(rename = "topology", skip_serializing_if = "Option::is_none")]
    pub topology: Option<CpuTopology>,

    /// 缓存配置
    #[serde(rename = "cache", skip_serializing_if = "Option::is_none")]
    pub cache: Option<CpuCache>,

    /// 最大物理地址配置
    #[serde(rename = "maxphysaddr", skip_serializing_if = "Option::is_none")]
    pub max_phys_addr: Option<MaxPhysAddr>,

    /// CPU 特性列表
    #[serde(rename = "feature", skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<CpuFeature>>,
}

impl CpuConfig {
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置自定义模式配置
    pub fn custom() -> Self {
        Self::new()
    }

    /// 设置 host-model 模式
    pub fn host_model() -> Self {
        Self {
            mode: Some(CpuMode::HostModel),
            ..Default::default()
        }
    }

    /// 设置 host-passthrough 模式
    pub fn host_passthrough() -> Self {
        Self {
            mode: Some(CpuMode::HostPassthrough),
            migratable: Some(false),
            ..Default::default()
        }
    }

    /// 设置 maximum 模式
    pub fn maximum() -> Self {
        Self {
            mode: Some(CpuMode::Maximum),
            migratable: Some(false),
            ..Default::default()
        }
    }

    /// 设置匹配模式
    pub fn with_match_mode(mut self, match_mode: CpuMatch) -> Self {
        self.match_mode = Some(match_mode);
        self
    }

    /// 设置 CPU 模式
    pub fn with_mode(mut self, mode: CpuMode) -> Self {
        self.mode = Some(mode);
        self
    }

    /// 设置可迁移性
    pub fn with_migratable(mut self, migratable: bool) -> Self {
        self.migratable = Some(migratable);
        self
    }

    /// 设置 CPU 模型
    pub fn with_model(mut self, model: CpuModel) -> Self {
        self.model = Some(model);
        self
    }

    /// 设置供应商
    pub fn with_vendor(mut self, vendor: &str) -> Self {
        self.vendor = Some(vendor.to_string());
        self
    }

    /// 设置拓扑配置
    pub fn with_topology(mut self, topology: CpuTopology) -> Self {
        self.topology = Some(topology);
        self
    }

    /// 添加 CPU 特性
    pub fn add_feature(mut self, feature: CpuFeature) -> Self {
        if self.features.is_none() {
            self.features = Some(Vec::new());
        }

        if let Some(ref mut features) = self.features {
            // 避免重复添加相同名称的特性
            if !features.iter().any(|f| f.name == feature.name) {
                features.push(feature);
            }
        }

        self
    }

    /// 批量添加特性
    pub fn with_features(mut self, features: Vec<CpuFeature>) -> Self {
        self.features = Some(features);
        self
    }

    /// 设置缓存配置
    pub fn with_cache(mut self, cache: CpuCache) -> Self {
        self.cache = Some(cache);
        self
    }

    /// 设置最大物理地址配置
    pub fn with_max_phys_addr(mut self, max_phys_addr: MaxPhysAddr) -> Self {
        self.max_phys_addr = Some(max_phys_addr);
        self
    }

    /// 验证配置
    pub fn validate(&self, max_vcpus: Option<u32>) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // 检查模式兼容性
        if let Some(mode) = &self.mode {
            match mode {
                CpuMode::HostModel | CpuMode::HostPassthrough | CpuMode::Maximum => {
                    // 这些模式不能与 match 属性同时使用
                    if self.match_mode.is_some() {
                        errors.push(format!(
                            "match attribute cannot be used with mode='{}'",
                            mode
                        ));
                    }
                }
                CpuMode::Custom => {
                    // 自定义模式可以使用 match 属性
                }
            }
        }

        // 验证 CPU 模型
        if let Some(model) = &self.model {
            match model.validate() {
                Ok(_) => {}
                Err(mut model_errors) => {
                    errors.append(&mut model_errors);
                }
            }
        }

        // 验证拓扑配置
        if let Some(topology) = &self.topology {
            match topology.validate(max_vcpus) {
                Ok(_) => {}
                Err(mut topology_errors) => {
                    errors.append(&mut topology_errors);
                }
            }
        }

        // 验证特性列表
        if let Some(features) = &self.features {
            for (index, feature) in features.iter().enumerate() {
                match feature.validate() {
                    Ok(_) => {}
                    Err(mut feature_errors) => {
                        errors.push(format!("Feature {} (index {}):", feature.name, index));
                        errors.append(&mut feature_errors);
                    }
                }
            }

            // 检查重复特性
            let mut seen = HashMap::new();
            for (index, feature) in features.iter().enumerate() {
                if let Some(prev_index) = seen.insert(&feature.name, index) {
                    errors.push(format!(
                        "Duplicate CPU feature '{}' at positions {} and {}",
                        feature.name,
                        prev_index + 1,
                        index + 1
                    ));
                }
            }
        }

        // 验证缓存配置
        if let Some(cache) = &self.cache {
            match cache.validate() {
                Ok(_) => {}
                Err(mut cache_errors) => {
                    errors.append(&mut cache_errors);
                }
            }
        }

        // 验证最大物理地址配置
        if let Some(max_phys_addr) = &self.max_phys_addr {
            match max_phys_addr.validate() {
                Ok(_) => {}
                Err(mut max_phys_errors) => {
                    errors.append(&mut max_phys_errors);
                }
            }
        }

        // 检查供应商长度
        if let Some(vendor) = &self.vendor
            && vendor.is_empty()
        {
            errors.push("Vendor string cannot be empty if specified".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// 获取配置摘要
    pub fn get_summary(&self) -> String {
        let mut summary = String::from("CPU Configuration:\n");

        // 模式信息
        summary.push_str(&format!(
            "  Mode: {}\n",
            self.mode
                .map(|m| m.to_string())
                .unwrap_or_else(|| "custom".to_string())
        ));

        if let Some(match_mode) = &self.match_mode {
            summary.push_str(&format!("  Match: {}\n", match_mode));
        }

        if let Some(check) = &self.check {
            summary.push_str(&format!("  Check: {:?}\n", check));
        }

        if let Some(migratable) = self.migratable {
            summary.push_str(&format!("  Migratable: {}\n", migratable));
        }

        // 模型信息
        if let Some(model) = &self.model {
            summary.push_str(&format!("  Model: {}\n", model.name));
            if let Some(fallback) = &model.fallback {
                summary.push_str(&format!("  Fallback: {:?}\n", fallback));
            }
            if let Some(vendor_id) = &model.vendor_id {
                summary.push_str(&format!("  Vendor ID: {}\n", vendor_id));
            }
        }

        // 供应商
        if let Some(vendor) = &self.vendor {
            summary.push_str(&format!("  Vendor: {}\n", vendor));
        }

        // 拓扑信息
        if let Some(topology) = &self.topology {
            let total_vcpus = topology.total_vcpus();
            summary.push_str(&format!(
                "  Topology: {} sockets, {} dies, {} clusters, {} cores, {} threads\n",
                topology.sockets,
                topology.dies.unwrap_or(1),
                topology.clusters.unwrap_or(1),
                topology.cores,
                topology.threads
            ));
            summary.push_str(&format!("  Total vCPUs: {}\n", total_vcpus));
        }

        // 特性信息
        if let Some(features) = &self.features {
            summary.push_str(&format!("  Features: {} configured\n", features.len()));

            let policy_counts: HashMap<FeaturePolicy, usize> =
                features.iter().fold(HashMap::new(), |mut acc, f| {
                    *acc.entry(f.policy).or_insert(0) += 1;
                    acc
                });

            for (policy, count) in policy_counts {
                summary.push_str(&format!("    {}: {}\n", policy, count));
            }
        }

        // 缓存信息
        if let Some(cache) = &self.cache {
            if let Some(level) = cache.level {
                summary.push_str(&format!("  Cache Level {}: {}\n", level, cache.mode));
            } else {
                summary.push_str(&format!("  Cache: {}\n", cache.mode));
            }
        }

        // 最大物理地址信息
        if let Some(max_phys_addr) = &self.max_phys_addr {
            summary.push_str(&format!("  Max Physical Address: {}\n", max_phys_addr.mode));
            if let Some(bits) = max_phys_addr.bits {
                summary.push_str(&format!("    Bits: {}\n", bits));
            }
            if let Some(limit) = max_phys_addr.limit {
                summary.push_str(&format!("    Limit: {}\n", limit));
            }
        }

        summary
    }

    /// 获取建议的配置（基于常见用例）
    pub fn suggested_config(scenario: &str) -> Option<Self> {
        match scenario.to_lowercase().as_str() {
            "high-performance" => Some(
                CpuConfig::host_passthrough()
                    .with_topology(CpuTopology::new(2, 8, 2))
                    .with_migratable(false),
            ),
            "balanced" => Some(
                CpuConfig::custom()
                    .with_model(CpuModel::new("host-model"))
                    .with_topology(CpuTopology::new(1, 4, 2))
                    .add_feature(CpuFeature::new("pcid", FeaturePolicy::Require)),
            ),
            "compatibility" => Some(
                CpuConfig::custom()
                    .with_match_mode(CpuMatch::Exact)
                    .with_model(CpuModel::new("Nehalem").with_fallback(FallbackPolicy::Forbid))
                    .with_topology(CpuTopology::new(1, 2, 1)),
            ),
            "virtualization-optimized" => Some(
                CpuConfig::custom()
                    .with_model(CpuModel::new("Skylake-Client"))
                    .with_topology(CpuTopology::new(2, 6, 1))
                    .add_feature(CpuFeature::new("vmx", FeaturePolicy::Require))
                    .add_feature(CpuFeature::new("svm", FeaturePolicy::Disable))
                    .with_cache(CpuCache::new(CacheMode::Passthrough)),
            ),
            _ => None,
        }
    }
}
