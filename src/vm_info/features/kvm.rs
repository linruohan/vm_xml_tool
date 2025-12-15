use super::FeatureState;
use serde::{Deserialize, Serialize};

// KVM 配置
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct KvmFeatures {
    #[serde(rename = "hidden", skip_serializing_if = "Option::is_none")]
    pub hidden: Option<FeatureState>,

    #[serde(rename = "hint-dedicated", skip_serializing_if = "Option::is_none")]
    pub hint_dedicated: Option<FeatureState>,

    #[serde(rename = "poll-control", skip_serializing_if = "Option::is_none")]
    pub poll_control: Option<FeatureState>,

    #[serde(rename = "pv-ipi", skip_serializing_if = "Option::is_none")]
    pub pv_ipi: Option<FeatureState>,

    #[serde(rename = "dirty-ring", skip_serializing_if = "Option::is_none")]
    pub dirty_ring: Option<DirtyRingConfig>,
}
/// Dirty Ring 配置
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DirtyRingConfig {
    #[serde(rename = "@state")]
    pub state: FeatureState,

    #[serde(rename = "@size", skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,
}
