mod fwcfg;
mod smbios;

use crate::vm_info::sysinfo::fwcfg::FwcfgSysinfo;
use crate::vm_info::sysinfo::smbios::SmbiosSysinfo;
use serde::{Deserialize, Serialize};
use smbios::{BaseBoardInfo, BiosInfo, ChassisInfo, OemStringsInfo, SystemInfo};

// sysinfo 解析有问题，暂时不能用
#[derive(Debug, Deserialize, Serialize)]
pub struct Sysinfo {
    #[serde(rename = "@type")]
    pub sysinfo_type: String, // "smbios" 或 "fwcfg"
    #[serde(flatten)]
    pub content: SysinfoContent,
}
#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct SysinfoRaw {
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
impl From<SysinfoRaw> for SysinfoContent {
    fn from(raw: SysinfoRaw) -> Self {
        match raw.sysinfo_type.as_str() {
            "fwcfg" => SysinfoContent::Fwcfg(
                FwcfgSysinfo {
                    sysinfo_type: raw.sysinfo_type,
                    entries: raw.entities.unwrap_or_default(),
                }),
            "smbios" => SysinfoContent::Smbios(SmbiosSysinfo {
                sysinfo_type: raw.sysinfo_type,
                bios: raw.bios,
                system: raw.system,
                base_board: raw.base_board,
                chassis: raw.chassis,
                oem_strings: raw.oem_strings,
            }),
            _ => panic!("unknown type"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "sysinfo")]
pub enum SysinfoContent {
    #[serde(rename = "smbios")]
    Smbios(smbios::SmbiosSysinfo),
    #[serde(rename = "fwcfg")]
    Fwcfg(fwcfg::FwcfgSysinfo),
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Sysinfo2 {
    #[serde(rename = "smbios")]
    Smbios(smbios::SmbiosSysinfo),
    #[serde(rename = "fwcfg")]
    Fwcfg(fwcfg::FwcfgSysinfo),
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SysinfoEntry {
    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@file", skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,

    #[serde(rename = "$text")]
    pub value: String,
}
