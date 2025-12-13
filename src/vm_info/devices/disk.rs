use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Disk {
    #[serde(rename = "@type")]
    pub disk_type: String,
    #[serde(rename = "@device")]
    pub device: String,
    pub driver: Driver,
    pub source: Source,
    pub target: Target,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Driver {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@type")]
    pub driver_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    #[serde(rename = "@file")]
    pub file: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Target {
    #[serde(rename = "@dev")]
    pub dev: String,
    #[serde(rename = "@bus")]
    pub bus: String,
}
