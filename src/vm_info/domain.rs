use super::memory::{CurrentMemory, MaxMemory};
use super::{Cputune, Devices, Memory, MemoryBacking, MetaData, Os, Sysinfo, Vcpu, Vcpus};
use crate::vm_info::memtune::Memtune;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "domain")]
pub struct Domain {
    #[serde(rename = "@type")]
    pub domain_type: String,
    // 基本信息
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
    // 内存
    pub memory: Memory,
    #[serde(rename = "maxMemory", skip_serializing_if = "Option::is_none")]
    pub max_memory: Option<MaxMemory>,
    #[serde(rename = "currentMemory", skip_serializing_if = "Option::is_none")]
    pub current_memory: Option<CurrentMemory>,
    // cpu
    pub vcpu: Vcpu, //虚拟机最大cpu
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcpus: Option<Vcpus>, //控制每个vcpu的状态
    pub cputune: Option<Cputune>, // cpu可调参数
    // 引导
    pub os: Os, // 虚拟机的引导
    #[serde(rename = "sysinfo", skip_serializing_if = "Option::is_none")]
    pub sysinfo: Option<Vec<Sysinfo>>,
    #[serde(rename = "memoryBacking", skip_serializing_if = "Option::is_none")]
    pub memory_backing: Option<MemoryBacking>, // 虚拟内存页
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memtune: Option<Memtune>,
    // 设备
    pub devices: Devices,
}
