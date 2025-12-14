use super::{Memory, MetaData, Os, Sysinfo, Vcpu, Vcpus};
use crate::Devices;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "domain")]
pub struct Domain {
    #[serde(rename = "@type")]
    pub domain_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MetaData>,

    pub name: String,
    pub memory: Memory,
    pub vcpu: Vcpu, //虚拟机最大cpu
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcpus: Option<Vcpus>, //控制每个vcpu的状态
    pub os: Os, // 虚拟机的引导
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sysinfo: Option<Vec<Sysinfo>>,
    pub devices: Devices,
}
