use serde::{Deserialize, Serialize};

// SMBIOS 类型的 sysinfo
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SmbiosSysinfo {
    pub bios: Option<BiosInfo>,
    pub system: Option<SystemInfo>,
    pub base_board: Option<BaseBoardInfo>,
    pub chassis: Option<ChassisInfo>,
    pub oem_strings: Option<OemStringsInfo>,
}

// BIOS 信息
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct BiosInfo {
    #[serde(rename = "entry", default)]
    pub entries: Vec<SysinfoEntry>,
}

// 系统信息
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SystemInfo {
    #[serde(rename = "entry", default)]
    pub entries: Vec<SysinfoEntry>,
}

// 主板信息
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct BaseBoardInfo {
    #[serde(rename = "entry", default)]
    pub entries: Vec<SysinfoEntry>,
}

// 机箱信息
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ChassisInfo {
    #[serde(rename = "entry", default)]
    pub entries: Vec<SysinfoEntry>,
}

// OEM 字符串信息
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OemStringsInfo {
    #[serde(rename = "entry", default)]
    pub entries: Vec<OemStringEntry>,
}

// 通用 sysinfo 条目
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SysinfoEntry {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$text")]
    pub value: String,
}

// OEM 字符串条目（无 name 属性）
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OemStringEntry {
    #[serde(rename = "$text")]
    pub value: String,
}
