// 另一种解析sysinfo方式

use crate::vm_info::sysinfo::fwcfg::FwcfgSysinfo;
use crate::vm_info::sysinfo::smbios::{
    BaseBoardInfo, BiosInfo, ChassisInfo, OemStringsInfo, SmbiosSysinfo, SystemInfo,
};
use crate::vm_info::sysinfo::{fwcfg, smbios};
use serde::{Deserialize, Serialize};

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

//
fn run() {
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename = "domain")]
    pub struct Domain {
        #[serde(rename = "sysinfo", skip_serializing_if = "Option::is_none")]
        pub sysinfo: Option<Vec<RawSysinfo>>,
    }
}