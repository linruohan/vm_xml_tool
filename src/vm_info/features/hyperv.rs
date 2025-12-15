use crate::vm_info::features::FeatureState;
use serde::{Deserialize, Serialize};

/// Hyper-V 模式
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum HypervMode {
    Custom,
    Default,
}

/// Hyper-V 特性配置
#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct HypervFeatures {
    #[serde(rename = "@mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<HypervMode>,

    /// Relaxed timing
    #[serde(rename = "relaxed", skip_serializing_if = "Option::is_none")]
    pub relaxed: Option<FeatureState>,

    /// Virtual APIC
    #[serde(rename = "vapic", skip_serializing_if = "Option::is_none")]
    pub vapic: Option<FeatureState>,

    /// Spinlocks
    #[serde(rename = "spinlocks", skip_serializing_if = "Option::is_none")]
    pub spinlocks: Option<SpinlocksConfig>,

    /// VP (Virtual Processor) index
    #[serde(rename = "vpindex", skip_serializing_if = "Option::is_none")]
    pub vpindex: Option<FeatureState>,

    /// Runtime
    #[serde(rename = "runtime", skip_serializing_if = "Option::is_none")]
    pub runtime: Option<FeatureState>,

    /// Synthetic interrupt controller
    #[serde(rename = "synic", skip_serializing_if = "Option::is_none")]
    pub synic: Option<FeatureState>,

    /// Synthetic timer
    #[serde(rename = "stimer", skip_serializing_if = "Option::is_none")]
    pub stimer: Option<StimerConfig>,

    /// Reset
    #[serde(rename = "reset", skip_serializing_if = "Option::is_none")]
    pub reset: Option<FeatureState>,

    /// Vendor ID
    #[serde(rename = "vendor_id", skip_serializing_if = "Option::is_none")]
    pub vendor_id: Option<VendorIdConfig>,

    /// Frequencies
    #[serde(rename = "frequencies", skip_serializing_if = "Option::is_none")]
    pub frequencies: Option<FeatureState>,

    /// Re-enlightenment
    #[serde(rename = "reenlightenment", skip_serializing_if = "Option::is_none")]
    pub reenlightenment: Option<FeatureState>,

    /// TLB flush
    #[serde(rename = "tlbflush", skip_serializing_if = "Option::is_none")]
    pub tlbflush: Option<TlbFlushConfig>,

    /// IPI (Inter-Processor Interrupt)
    #[serde(rename = "ipi", skip_serializing_if = "Option::is_none")]
    pub ipi: Option<FeatureState>,

    /// Enlightened VMCS (Virtual Machine Control Structure)
    #[serde(rename = "evmcs", skip_serializing_if = "Option::is_none")]
    pub evmcs: Option<FeatureState>,

    /// Extended MSR bitmap
    #[serde(rename = "emsr_bitmap", skip_serializing_if = "Option::is_none")]
    pub emsr_bitmap: Option<FeatureState>,

    /// XMM (SSE) input state
    #[serde(rename = "xmm_input", skip_serializing_if = "Option::is_none")]
    pub xmm_input: Option<FeatureState>,
}

/// Spinlocks 配置
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SpinlocksConfig {
    #[serde(rename = "@state")]
    pub state: FeatureState,

    #[serde(rename = "@retries", skip_serializing_if = "Option::is_none")]
    pub retries: Option<u32>,
}

/// Synthetic Timer 配置
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct StimerConfig {
    #[serde(rename = "@state")]
    pub state: FeatureState,

    #[serde(rename = "direct", skip_serializing_if = "Option::is_none")]
    pub direct: Option<FeatureState>,
}

/// Vendor ID 配置
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct VendorIdConfig {
    #[serde(rename = "@state")]
    pub state: FeatureState,

    #[serde(rename = "@value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// TLB Flush 配置
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TlbFlushConfig {
    #[serde(rename = "@state")]
    pub state: FeatureState,

    #[serde(rename = "direct", skip_serializing_if = "Option::is_none")]
    pub direct: Option<FeatureState>,

    #[serde(rename = "extended", skip_serializing_if = "Option::is_none")]
    pub extended: Option<FeatureState>,
}
