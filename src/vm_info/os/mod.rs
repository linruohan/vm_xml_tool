mod loader;
mod nvram;
mod smbios;

use serde::{Deserialize, Serialize};

use loader::Loader;
use nvram::Nvram;
use smbios::Smbios;
// Bios bootloader

#[derive(Debug, Deserialize, Serialize)]
pub struct Os {
    // 属性
    #[serde(rename = "@firmware", skip_serializing_if = "Option::is_none")]
    pub firmware: Option<String>, // bios和 efi

    // 子元素
    #[serde(rename = "type")]
    pub os_type: OsType, // hvm 裸机完全虚拟化；linux

    #[serde(rename = "loader", skip_serializing_if = "Option::is_none")]
    pub loader: Option<Loader>,

    #[serde(rename = "nvram", skip_serializing_if = "Option::is_none")]
    pub nvram: Option<Nvram>,

    #[serde(rename = "boot", skip_serializing_if = "Vec::is_empty")]
    pub boots: Vec<Boot>,

    #[serde(rename = "bootmenu", skip_serializing_if = "Option::is_none")]
    pub bootmenu: Option<BootMenu>,

    #[serde(rename = "smbios", skip_serializing_if = "Option::is_none")]
    pub smbios: Option<Smbios>,

    #[serde(rename = "bios", skip_serializing_if = "Option::is_none")]
    pub bios: Option<Bios>,
}

// 指定要在虚拟机中引导的操作系统类型：hvm 完全虚拟化; linux;
#[derive(Debug, Deserialize, Serialize)]
pub struct OsType {
    // 属性
    #[serde(rename = "@arch", skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,

    // 内容
    #[serde(rename = "$text")]
    pub value: String,
}

// 引导选项
#[derive(Debug, Deserialize, Serialize)]
pub struct Boot {
    #[serde(rename = "@dev")]
    pub dev: String, // e.g., "hd", "cdrom" fd  network
}

// 来宾启动时启用交互式启动菜单提示。enable属性可以是“yes”或“no”。未指定默认hypervisor
#[derive(Debug, Deserialize, Serialize)]
pub struct BootMenu {
    #[serde(rename = "@enable")]
    pub enable: String, // "yes" or "no"

    #[serde(rename = "@timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>, // [0,65535]
}

// 启用或禁用串行图形适配器，允许用户在串行端口上查看BIOS消息。
#[derive(Debug, Deserialize, Serialize)]
pub struct Bios {
    #[serde(rename = "@useserial")]
    pub useserial: String, // "yes" or "no"

    #[serde(rename = "@rebootTimeout", skip_serializing_if = "Option::is_none")]
    pub reboot_timeout: Option<String>,
}
