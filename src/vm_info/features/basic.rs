use super::{FeatureState, ValueAttribute};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct EmptyElement;
/// 基本 CPU 特性
#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct BasicCpuFeatures {
    /// Physical Address Extension
    #[serde(rename = "pae", skip_serializing_if = "Option::is_none")]
    pub pae: Option<EmptyElement>,

    /// Advanced Configuration and Power Interface
    #[serde(rename = "acpi", skip_serializing_if = "Option::is_none")]
    pub acpi: Option<EmptyElement>,

    /// Advanced Programmable Interrupt Controller
    #[serde(rename = "apic", skip_serializing_if = "Option::is_none")]
    pub apic: Option<EmptyElement>,

    /// Hardware Assisted Paging
    #[serde(rename = "hap", skip_serializing_if = "Option::is_none")]
    pub hap: Option<EmptyElement>,

    /// Private Network
    #[serde(rename = "privnet", skip_serializing_if = "Option::is_none")]
    pub privnet: Option<EmptyElement>,

    /// Para-virtualized spinlocks
    #[serde(rename = "pvspinlock")]
    pub pvspinlock: Option<FeatureState>,

    /// Generic Interrupt Controller
    #[serde(rename = "gic", skip_serializing_if = "Option::is_none")]
    pub gic: Option<GicConfig>,

    /// I/O Advanced Programmable Interrupt Controller
    #[serde(rename = "ioapic", skip_serializing_if = "Option::is_none")]
    pub ioapic: Option<IoApicConfig>,

    /// Hash Page Table
    #[serde(rename = "hpt", skip_serializing_if = "Option::is_none")]
    pub hpt: Option<HptConfig>,

    /// Virtual Machine Core Info
    #[serde(rename = "vmcoreinfo")]
    pub vmcoreinfo: Option<FeatureState>,

    /// System Management Mode
    #[serde(rename = "smm", skip_serializing_if = "Option::is_none")]
    pub smm: Option<SmmConfig>,

    /// Hardware Transactional Memory
    #[serde(rename = "htm")]
    pub htm: Option<FeatureState>,

    /// Counter Cache Flush Assist
    #[serde(rename = "ccf-assist")]
    pub ccf_assist: Option<FeatureState>,

    /// Model Specific Registers
    #[serde(rename = "msrs", skip_serializing_if = "Option::is_none")]
    pub msrs: Option<MsrsConfig>,

    /// Cache Flush Parameter Control
    #[serde(rename = "cfpc", skip_serializing_if = "Option::is_none")]
    pub cfpc: Option<ValueAttribute>,

    /// Speculative Branch Bypass Control
    #[serde(rename = "sbbc", skip_serializing_if = "Option::is_none")]
    pub sbbc: Option<ValueAttribute>,

    /// Instruction Based Sampling
    #[serde(rename = "ibs", skip_serializing_if = "Option::is_none")]
    pub ibs: Option<ValueAttribute>,

    /// Trusted Computing Group
    #[serde(rename = "tcg", skip_serializing_if = "Option::is_none")]
    pub tcg: Option<TcgConfig>,

    /// Asynchronous Teardown
    #[serde(rename = "async-teardown", skip_serializing_if = "Option::is_none")]
    pub async_teardown: Option<AsyncTeardownConfig>,

    /// Reliability, Availability, and Serviceability
    #[serde(rename = "ras")]
    pub ras: Option<FeatureState>,

    /// PS/2 Keyboard/Mouse
    #[serde(rename = "ps2")]
    pub ps2: Option<FeatureState>,

    /// Advanced Interrupt Architecture
    #[serde(rename = "aia", skip_serializing_if = "Option::is_none")]
    pub aia: Option<ValueAttribute>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum IoApicDriver {
    #[default]
    Qemu,
    Kvm,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum HptResizing {
    Required,
    Optional,
}

/// CFPC (Cache Flush Parameter Control) 配置
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CfpcValue {
    Workaround,
    Fixed,
    #[serde(rename = "fixed-ccd")]
    FixedCcd,
    #[serde(rename = "fixed-na")]
    FixedNa,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CfpcConfig {
    #[serde(rename = "@value")]
    pub value: CfpcValue,
}

/// SBBC (Speculative Branch Bypass Control) 配置
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SbbcValue {
    Workaround,
    Fixed,
    #[serde(rename = "fixed-na")]
    FixedNa,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SbbcConfig {
    #[serde(rename = "@value")]
    pub value: SbbcValue,
}

/// IBS (Instruction Based Sampling) 配置
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum IbsValue {
    Workaround,
    Fixed,
    #[serde(rename = "fixed-na")]
    FixedNa,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct IbsConfig {
    #[serde(rename = "@value")]
    pub value: IbsValue,
}

/// 异步拆卸配置
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AsyncTeardownConfig {
    #[serde(rename = "@enabled")]
    pub enabled: bool,
}

/// AIA (Advanced Interrupt Architecture) 配置
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AiaValue {
    #[serde(rename = "aplic-imsic")]
    AplicImsic,
    #[serde(rename = "aplic-only")]
    AplicOnly,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AiaConfig {
    #[serde(rename = "@value")]
    pub value: AiaValue,
}

// GIC 配置
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GicConfig {
    #[serde(rename = "@version")]
    pub version: String,
}

// IOAPIC 配置
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct IoApicConfig {
    #[serde(rename = "@driver")]
    pub driver: String,
}

// HPT 配置
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct HptConfig {
    #[serde(rename = "@resizing")]
    pub resizing: String,

    #[serde(rename = "maxpagesize")]
    pub max_page_size: PageSizeConfig,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PageSizeConfig {
    #[serde(rename = "@unit")]
    pub unit: String,

    #[serde(rename = "$value")]
    pub value: u32,
}

// SMM 配置
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SmmConfig {
    #[serde(rename = "@state")]
    pub state: String,

    #[serde(rename = "tseg")]
    pub tseg: PageSizeConfig,
}

// MSRS 配置
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MsrsConfig {
    #[serde(rename = "@unknown")]
    pub unknown: String,
}

// TCG 配置
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TcgConfig {
    #[serde(rename = "tb-cache")]
    pub tb_cache: PageSizeConfig,
}
