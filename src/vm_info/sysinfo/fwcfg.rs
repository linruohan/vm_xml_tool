use super::SysinfoEntry;
use serde::{Deserialize, Serialize};

// FWCFG 类型的 sysinfo
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "sysinfo")]
pub struct FwcfgSysinfo {
    #[serde(rename = "@type")]
    pub sysinfo_type: String, //"fwcfg"
    #[serde(rename = "entry", default)]
    pub entries: Vec<SysinfoEntry>,
}

// FWCFG 条目
#[allow(unused)]
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FwcfgEntry {
    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@file", skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,

    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
