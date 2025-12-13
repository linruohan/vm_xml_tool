use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MetaData {
    #[serde(rename = "app1_foo", skip_serializing_if = "Option::is_none")]
    pub app1_foo: Option<AppElement>,

    #[serde(rename = "app2_bar", skip_serializing_if = "Option::is_none")]
    pub app2_bar: Option<AppElement>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AppElement {
    #[serde(rename = "@xmlns:app1", skip_serializing_if = "Option::is_none")]
    pub xmlns_app1: Option<String>,

    #[serde(rename = "@xmlns:app2", skip_serializing_if = "Option::is_none")]
    pub xmlns_app2: Option<String>,

    #[serde(rename = "$text")]
    pub content: String,
}