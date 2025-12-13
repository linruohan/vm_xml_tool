use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MetaData {
    #[serde(
        rename = "{http://app1.org/app1}foo",
        skip_serializing_if = "Option::is_none"
    )]
    pub app1_foo: Option<GenericElement>,

    #[serde(
        rename = "{http://app1.org/app2}bar",
        skip_serializing_if = "Option::is_none"
    )]
    pub app2_bar: Option<GenericElement>,

    // 允许其他未指定的元数据元素
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub other: Option<std::collections::HashMap<String, GenericElement>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GenericElement {
    #[serde(rename = "$text")]
    pub content: String,

    // 捕获所有属性
    #[serde(flatten)]
    pub attributes: std::collections::HashMap<String, String>,
}
