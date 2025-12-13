use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Smbios {
    #[serde(rename = "@mode")]
    pub mode: String, // e.g., "sysinfo"
}
