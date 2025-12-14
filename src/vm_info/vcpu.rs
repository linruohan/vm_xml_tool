use serde::{Deserialize, Serialize};

// vcpu
#[derive(Debug, Deserialize, Serialize)]
pub struct Vcpu {
    #[serde(rename = "@placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<String>,
    #[serde(rename = "@cpuset", skip_serializing_if = "Option::is_none")]
    pub cpuset: Option<String>,
    #[serde(rename = "@current", skip_serializing_if = "Option::is_none")]
    pub current: Option<u32>,
    #[serde(rename = "$text")]
    pub vcpu_count: u32,
}
// vcpus
#[derive(Debug, Deserialize, Serialize)]
pub struct Vcpus{
    pub vcpu:Vec<VcpusItem>
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "vcpu")]
pub struct VcpusItem {
    #[serde(rename = "@id")]
    pub id: u32,
    #[serde(rename = "@enabled")]
    pub enabled: String, // "yes" or "no"
    #[serde(rename = "@hotpluggable")]
    pub hotpluggable: String, // "yes" or "no"
    #[serde(rename = "@order", skip_serializing_if = "Option::is_none")]
    pub order: Option<u32>,
}

