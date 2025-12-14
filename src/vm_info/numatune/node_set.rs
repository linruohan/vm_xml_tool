use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, de};
use std::collections::HashSet;
use std::fmt;
/// NUMA 节点集（支持复杂的节点表达式）
#[derive(Debug, PartialEq, Clone)]
pub struct NodeSet {
    pub expression: String,
}

// 为 NumaNodeset 实现 Serialize
impl Serialize for NodeSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.expression)
    }
}

// 为 NumaNodeset 实现 Deserialize
impl<'de> Deserialize<'de> for NodeSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NumaNodesetVisitor;

        impl<'de> Visitor<'de> for NumaNodesetVisitor {
            type Value = NodeSet;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing NUMA nodeset")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(NodeSet::new(value))
            }
        }

        deserializer.deserialize_str(NumaNodesetVisitor)
    }
}
impl NodeSet {
    /// 创建新的节点集
    pub fn new(expression: &str) -> Self {
        Self {
            expression: expression.to_string(),
        }
    }

    /// 解析节点集表达式
    pub fn parse(&self) -> Result<HashSet<u32>, String> {
        let mut nodes = HashSet::new();
        let parts: Vec<&str> = self.expression.split(',').collect();

        for part in parts {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }

            // 处理排除标记 (^)
            if let Some(node_str) = part.strip_prefix('^') {
                if let Ok(node) = node_str.parse::<u32>() {
                    nodes.remove(&node);
                } else {
                    return Err(format!("无效的节点号: {}", node_str));
                }
                continue;
            }

            // 处理范围 (如 "1-4")
            if part.contains('-') {
                let range_parts: Vec<&str> = part.split('-').collect();
                if range_parts.len() != 2 {
                    return Err(format!("无效的范围表达式: {}", part));
                }

                let start = range_parts[0]
                    .parse::<u32>()
                    .map_err(|_| format!("无效的起始节点: {}", range_parts[0]))?;
                let end = range_parts[1]
                    .parse::<u32>()
                    .map_err(|_| format!("无效的结束节点: {}", range_parts[1]))?;

                if start > end {
                    return Err(format!("起始节点不能大于结束节点: {}", part));
                }

                for node in start..=end {
                    nodes.insert(node);
                }
            } else {
                // 单个节点
                let node = part
                    .parse::<u32>()
                    .map_err(|_| format!("无效的节点号: {}", part))?;
                nodes.insert(node);
            }
        }

        Ok(nodes)
    }

    /// 验证节点集是否有效
    pub fn validate(&self, max_nodes: Option<u32>) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        match self.parse() {
            Ok(nodes) => {
                if nodes.is_empty() {
                    errors.push("节点集不能为空".to_string());
                }

                if let Some(max) = max_nodes {
                    for &node in &nodes {
                        if node >= max {
                            errors.push(format!("节点 {} 超出最大节点数 {}", node, max - 1));
                        }
                    }
                }
            }
            Err(e) => {
                errors.push(e);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// 获取节点数量
    pub fn node_count(&self) -> Result<usize, String> {
        self.parse().map(|nodes| nodes.len())
    }
}
