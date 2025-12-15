use super::FeatureState;
use serde::{Deserialize, Serialize};

/// Xen passthrough 模式
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
#[serde(rename_all = "lowercase")]
pub enum XenPassthroughMode {
    #[serde(rename = "share_pt")]
    SharePt,
    #[serde(rename = "sync_pt")]
    SyncPt,
}

// Xen 配置
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct XenFeatures {
    #[serde(rename = "e820_host", skip_serializing_if = "Option::is_none")]
    pub e820_host: Option<FeatureState>,

    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Option<PassthroughConfig>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PassthroughConfig {
    #[serde(rename = "@state")]
    pub state: String,

    #[serde(rename = "@mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<XenPassthroughMode>,
}
