mod fwcfg;
mod smbios;

use crate::vm_info::sysinfo::fwcfg::FwcfgSysinfo;
use crate::vm_info::sysinfo::smbios::{BaseBoardInfo, BiosInfo, ChassisInfo, OemStringsInfo, SmbiosSysinfo, SystemInfo};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Sysinfo0 {
    #[serde(rename = "@type")]
    pub sysinfo_type: String, // "smbios" 或 "fwcfg"
    #[serde(flatten)]
    pub content: SysinfoContent,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SysinfoContent {
    #[serde(rename = "smbios")]
    Smbios(smbios::SmbiosSysinfo),
    #[serde(rename = "fwcfg")]
    Fwcfg(fwcfg::FwcfgSysinfo),
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct RawSysinfo {
    #[serde(rename = "@type")]
    sysinfo_type: String,
    bios: Option<BiosInfo>,
    system: Option<SystemInfo>,
    #[serde(rename = "baseBoard")]
    base_board: Option<BaseBoardInfo>,
    chassis: Option<ChassisInfo>,
    #[serde(rename = "oemStrings")]
    oem_strings: Option<OemStringsInfo>,

    #[serde(rename = "entry", default)]
    entries: Vec<SysinfoEntry>,
}

impl From<RawSysinfo> for Sysinfo {
    fn from(raw: RawSysinfo) -> Self {
        match raw.sysinfo_type.as_str() {
            "fwcfg" => Sysinfo::Fwcfg(FwcfgSysinfo {
                sysinfo_type: raw.sysinfo_type,
                entries: raw.entries,
            }),
            "smbios" => Sysinfo::Smbios(SmbiosSysinfo {
                sysinfo_type: raw.sysinfo_type,
                bios: raw.bios,
                system: raw.system,
                base_board: raw.base_board,
                chassis: raw.chassis,
                oem_strings: raw.oem_strings,
            }),
            _ => panic!("Unknown sysinfo type"),
        }
    }
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Sysinfo {
    #[serde(rename = "fwcfg")]
    Fwcfg(fwcfg::FwcfgSysinfo),
    #[serde(rename = "smbios")]
    Smbios(smbios::SmbiosSysinfo),
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SysinfoEntry {
    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@file", skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,

    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
