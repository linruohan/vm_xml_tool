use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Os {
    #[serde(rename = "type")]
    pub os_type: Type,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Type {
    #[serde(rename = "@arch")]
    pub arch: String,
    #[serde(rename = "$text")]
    pub value: String,
}
