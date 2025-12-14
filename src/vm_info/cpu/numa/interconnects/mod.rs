use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

mod bandwidth;
mod latency;
use bandwidth::BandwidthConfig;
use latency::LatencyConfig;

/// 互连访问类型
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Hash)]
#[serde(rename_all = "lowercase")]
pub enum InterconnectType {
    Access,
    Read,
    Write,
}

/// 互连配置
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct InterconnectConfig {
    #[serde(rename = "latency", skip_serializing_if = "Option::is_none")]
    pub latencies: Option<Vec<LatencyConfig>>,

    #[serde(rename = "bandwidth", skip_serializing_if = "Option::is_none")]
    pub bandwidths: Option<Vec<BandwidthConfig>>,
}

impl InterconnectConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_latency(mut self, latency: LatencyConfig) -> Self {
        if self.latencies.is_none() {
            self.latencies = Some(Vec::new());
        }

        if let Some(ref mut latencies) = self.latencies {
            latencies.push(latency);
        }

        self
    }

    pub fn add_bandwidth(mut self, bandwidth: BandwidthConfig) -> Self {
        if self.bandwidths.is_none() {
            self.bandwidths = Some(Vec::new());
        }

        if let Some(ref mut bandwidths) = self.bandwidths {
            bandwidths.push(bandwidth);
        }

        self
    }

    pub fn validate(&self, total_cells: u32) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // 验证延迟配置
        if let Some(latencies) = &self.latencies {
            for latency in latencies {
                match latency.validate(total_cells) {
                    Ok(_) => {}
                    Err(mut latency_errors) => {
                        errors.append(&mut latency_errors);
                    }
                }
            }

            // 检查重复的配置
            let mut seen = HashSet::new();
            for latency in latencies {
                let key = (
                    latency.initiator,
                    latency.target,
                    latency.latency_type,
                    latency.cache_level,
                );
                if !seen.insert(key) {
                    errors.push(format!(
                        "Duplicate latency configuration for initiator={}, target={}, type={:?}, cache={:?}",
                        latency.initiator,
                        latency.target,
                        latency.latency_type,
                        latency.cache_level
                    ));
                }
            }
        }

        // 验证带宽配置
        if let Some(bandwidths) = &self.bandwidths {
            for bandwidth in bandwidths {
                match bandwidth.validate(total_cells) {
                    Ok(_) => {}
                    Err(mut bandwidth_errors) => {
                        errors.append(&mut bandwidth_errors);
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

    /// 获取延迟矩阵
    pub fn get_latency_matrix(&self, _total_cells: u32) -> HashMap<(u32, u32), u64> {
        let mut matrix = HashMap::new();

        if let Some(latencies) = &self.latencies {
            for latency in latencies {
                if latency.cache_level.is_none() {
                    matrix.insert((latency.initiator, latency.target), latency.value);
                }
            }
        }

        matrix
    }

    /// 获取带宽矩阵
    pub fn get_bandwidth_matrix(&self) -> HashMap<(u32, u32), u64> {
        let mut matrix = HashMap::new();

        if let Some(bandwidths) = &self.bandwidths {
            for bandwidth in bandwidths {
                matrix.insert(
                    (bandwidth.initiator, bandwidth.target),
                    bandwidth.to_kib_per_sec(),
                );
            }
        }

        matrix
    }
}
