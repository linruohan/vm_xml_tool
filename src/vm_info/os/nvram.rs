// 一些UEFI固件可能希望使用非易失性存储器来存储一些变量。在主机中，这表示为文件，并且文件的绝对路径存储在此元素中。此外，当域启动时，libvirt复制所谓的主NVRAM存储文件，由固件自动选择过程选择或在qemu.conf中定义。如果需要，可以使用template属性来覆盖自动选择的NVRAM模板和templateFormat来指定模板文件的格式（目前支持的格式是raw和qcow2）。当使用固件自动选择时，templateFormat字段反映所选模板的格式。10.10.0起（仅限QEMU）
// 
// 
// 注意，对于临时域，如果NVRAM文件是由libvirt创建的，那么它将被遗留下来，保存和删除文件是管理应用程序的责任（如果需要持久化）。1.2.8以来
// 
// 
// 从8.5.0开始，元素可以具有type属性（接受值file、block和network），在这种情况下，NVRAM存储由<source>子元素描述，其语法与磁盘源相同。参见硬盘驱动器，软盘，光盘。对于块支持的NVRAM映像，可能需要根据管理程序的期望确保块设备具有正确的来宾可见大小。这可能需要使用允许任意磁盘大小的非原始格式映像。
// 
// 
// 注意：网络支持的NVRAM变量不是从模板实例化的，用户有责任提供一个有效的NVRAM映像。
// 
// 
// 该元素支持format属性，该属性指定NVRAM映像的格式。注意，如果格式与templateFormat不同，管理程序可能不支持自动填充nvram，或者可能只支持特定的格式。
// 
// 
// 如果加载器被标记为无状态，则提供此元素无效。

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Nvram {
    // 属性
    #[serde(rename = "@template", skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,

    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub nvram_type: Option<String>, // "file", "network", etc.

    // 内容 - 可以是文本或复杂结构
    #[serde(flatten)]
    pub content: NvramContent,
}

// 处理 nvram 的不同内容类型
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum NvramContent {
    SimplePath(String), // 简单路径文本
    Complex(NvramComplex), // 复杂结构
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NvramComplex {
    // 可能包含 source 元素
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,

    // 其他可能的子元素
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub extra: HashMap<String, String>,

    // 文本内容（如果有）
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Source {
    // 属性
    #[serde(rename = "@file", skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,

    #[serde(rename = "@protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>, // e.g., "iscsi"

    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    // 子元素
    #[serde(rename = "host", skip_serializing_if = "Option::is_none")]
    pub host: Option<Host>,

    #[serde(rename = "auth", skip_serializing_if = "Option::is_none")]
    pub auth: Option<Auth>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Host {
    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@port", skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth {
    #[serde(rename = "@username")]
    pub username: String,

    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<Secret>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Secret {
    #[serde(rename = "@type")]
    pub secret_type: String, // e.g., "iscsi"

    #[serde(rename = "@usage")]
    pub usage: String,
}
