mod fwcfg;
mod smbios;

use fwcfg::FwcfgSysinfo;
use serde::{Deserialize, Serialize};
use smbios::SmbiosSysinfo;

#[derive(Debug, Deserialize, Serialize)]
pub struct Sysinfo {
    #[serde(rename = "@type")]
    pub sysinfo_type: String, // "smbios" 或 "fwcfg"

    // 根据类型选择不同的内容
    #[serde(flatten)]
    pub content: SysinfoContent,
}

// 使用枚举处理不同类型的 sysinfo 内容
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SysinfoContent {
    Smbios(SmbiosSysinfo),
    Fwcfg(FwcfgSysinfo),
}
