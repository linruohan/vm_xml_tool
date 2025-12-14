mod fwcfg;
mod smbios;

use crate::vm_info::sysinfo::smbios::{
    BaseBoardInfo, BiosInfo, ChassisInfo, OemStringsInfo, SystemInfo,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Sysinfo {
    #[serde(rename = "@type")]
    sysinfo_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    bios: Option<BiosInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<SystemInfo>,
    #[serde(rename = "baseBoard", skip_serializing_if = "Option::is_none")]
    base_board: Option<BaseBoardInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chassis: Option<ChassisInfo>,
    #[serde(rename = "oemStrings", skip_serializing_if = "Option::is_none")]
    oem_strings: Option<OemStringsInfo>,

    #[serde(rename = "entry", default)]
    entries: Vec<SysinfoEntry>,
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
