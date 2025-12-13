use serde::{Deserialize, Serialize};

// FWCFG 类型的 sysinfo
#[derive(Debug, Deserialize, Serialize)]
pub struct FwcfgSysinfo {
    #[serde(rename = "entry", default)]
    pub entries: Vec<FwcfgEntry>,
}

// FWCFG 条目
#[derive(Debug, Deserialize, Serialize)]
pub struct FwcfgEntry {
    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@file", skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,

    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
