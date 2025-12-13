use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Memory {
    #[serde(rename = "@unit")]
    pub unit: String,
    #[serde(rename = "$text")]
    pub value: u64,
}
