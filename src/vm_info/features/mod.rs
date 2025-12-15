use serde::{Deserialize, Serialize};

// 空元素标记
#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct EmptyElement;

// 基础功能状态
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct FeatureState {
    #[serde(rename = "@state")]
    pub state: String,
}

// 启用属性
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct EnabledAttribute {
    #[serde(rename = "@enabled")]
    pub enabled: String,
}

// 值属性
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ValueAttribute {
    #[serde(rename = "@value")]
    pub value: String,
}

// 页面大小配置
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PageSizeConfig {
    #[serde(rename = "@unit")]
    pub unit: String,

    #[serde(rename = "$value")]
    pub value: u32,
}

// Hyper-V 配置
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct HypervConfig {
    #[serde(rename = "@mode")]
    pub mode: String,

    #[serde(rename = "relaxed")]
    pub relaxed: FeatureState,

    #[serde(rename = "vapic")]
    pub vapic: FeatureState,

    #[serde(rename = "spinlocks")]
    pub spinlocks: Spinlocks,

    #[serde(rename = "vpindex")]
    pub vpindex: FeatureState,

    #[serde(rename = "runtime")]
    pub runtime: FeatureState,

    #[serde(rename = "synic")]
    pub synic: FeatureState,

    #[serde(rename = "stimer")]
    pub stimer: Stimer,

    #[serde(rename = "reset")]
    pub reset: FeatureState,

    #[serde(rename = "vendor_id")]
    pub vendor_id: VendorId,

    #[serde(rename = "frequencies")]
    pub frequencies: FeatureState,

    #[serde(rename = "reenlightenment")]
    pub reenlightenment: FeatureState,

    #[serde(rename = "tlbflush")]
    pub tlbflush: TlbFlush,

    #[serde(rename = "ipi")]
    pub ipi: FeatureState,

    #[serde(rename = "evmcs")]
    pub evmcs: FeatureState,

    #[serde(rename = "emsr_bitmap")]
    pub emsr_bitmap: FeatureState,

    #[serde(rename = "xmm_input")]
    pub xmm_input: FeatureState,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Spinlocks {
    #[serde(rename = "@state")]
    pub state: String,

    #[serde(rename = "@retries")]
    pub retries: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Stimer {
    #[serde(rename = "@state")]
    pub state: String,

    #[serde(rename = "direct")]
    pub direct: FeatureState,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct VendorId {
    #[serde(rename = "@state")]
    pub state: String,

    #[serde(rename = "@value")]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TlbFlush {
    #[serde(rename = "@state")]
    pub state: String,

    #[serde(rename = "direct")]
    pub direct: FeatureState,

    #[serde(rename = "extended")]
    pub extended: FeatureState,
}

// KVM 配置
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct KvmConfig {
    #[serde(rename = "hidden")]
    pub hidden: FeatureState,

    #[serde(rename = "hint-dedicated")]
    pub hint_dedicated: FeatureState,

    #[serde(rename = "poll-control")]
    pub poll_control: FeatureState,

    #[serde(rename = "pv-ipi")]
    pub pv_ipi: FeatureState,

    #[serde(rename = "dirty-ring")]
    pub dirty_ring: DirtyRing,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DirtyRing {
    #[serde(rename = "@state")]
    pub state: String,

    #[serde(rename = "@size")]
    pub size: u32,
}

// Xen 配置
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct XenConfig {
    #[serde(rename = "e820_host")]
    pub e820_host: FeatureState,

    #[serde(rename = "passthrough")]
    pub passthrough: Passthrough,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Passthrough {
    #[serde(rename = "@state")]
    pub state: String,

    #[serde(rename = "@mode")]
    pub mode: String,
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
    pub maxpagesize: PageSizeConfig,
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

// 顶层 Features 结构
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Features {
    #[serde(rename = "pae")]
    pub pae: EmptyElement,

    #[serde(rename = "acpi")]
    pub acpi: EmptyElement,

    #[serde(rename = "apic")]
    pub apic: EmptyElement,

    #[serde(rename = "hap")]
    pub hap: EmptyElement,

    #[serde(rename = "privnet")]
    pub privnet: EmptyElement,

    #[serde(rename = "hyperv")]
    pub hyperv: HypervConfig,

    #[serde(rename = "kvm")]
    pub kvm: KvmConfig,

    #[serde(rename = "xen")]
    pub xen: XenConfig,

    #[serde(rename = "pvspinlock")]
    pub pvspinlock: FeatureState,

    #[serde(rename = "gic")]
    pub gic: GicConfig,

    #[serde(rename = "ioapic")]
    pub ioapic: IoApicConfig,

    #[serde(rename = "hpt")]
    pub hpt: HptConfig,

    #[serde(rename = "vmcoreinfo")]
    pub vmcoreinfo: FeatureState,

    #[serde(rename = "smm")]
    pub smm: SmmConfig,

    #[serde(rename = "htm")]
    pub htm: FeatureState,

    #[serde(rename = "ccf-assist")]
    pub ccf_assist: FeatureState,

    #[serde(rename = "msrs")]
    pub msrs: MsrsConfig,

    #[serde(rename = "cfpc")]
    pub cfpc: ValueAttribute,

    #[serde(rename = "sbbc")]
    pub sbbc: ValueAttribute,

    #[serde(rename = "ibs")]
    pub ibs: ValueAttribute,

    #[serde(rename = "tcg")]
    pub tcg: TcgConfig,

    #[serde(rename = "async-teardown")]
    pub async_teardown: EnabledAttribute,

    #[serde(rename = "ras")]
    pub ras: FeatureState,

    #[serde(rename = "ps2")]
    pub ps2: FeatureState,

    #[serde(rename = "aia")]
    pub aia: ValueAttribute,
}
