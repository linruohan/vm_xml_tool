mod fwcfg;
mod smbios;

pub use fwcfg::FwcfgSysinfo;
use serde::{Deserialize, Serialize};
pub use smbios::SmbiosSysinfo;
use smbios::{BaseBoardInfo, BiosInfo, ChassisInfo, OemStringsInfo, SystemInfo};

// sysinfo 解析有问题，暂时不能用
#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Sysinfo {
    #[serde(rename = "@type")]
    pub sysinfo_type: String, // "smbios" 或 "fwcfg"

    // SmbiosSysinfo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bios: Option<BiosInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<SystemInfo>,
    #[serde(rename = "baseBoard", skip_serializing_if = "Option::is_none")]
    pub base_board: Option<BaseBoardInfo>,
    #[serde(rename = "chassis", skip_serializing_if = "Option::is_none")]
    pub chassis: Option<ChassisInfo>,
    #[serde(rename = "oemStrings", skip_serializing_if = "Option::is_none")]
    pub oem_strings: Option<OemStringsInfo>,

    // FwcfgSysinfo
    #[serde(rename = "entry", skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<SysinfoEntry>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SysinfoEntry {
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "@file", skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,

    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
