use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::{Path, PathBuf};

/// 分区路径 - 封装路径验证逻辑
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PartitionConfig {
    /// 分区路径
    #[serde(rename = "$value")]
    pub path: PartitionPath,
}
impl PartitionConfig {
    /// 创建新的分区配置
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: PartitionPath::new(path),
        }
    }

    /// 验证分区配置
    pub fn validate(&self) -> Result<(), Vec<String>> {
        self.path.validate()
    }

    /// 获取分区路径字符串
    pub fn get_path(&self) -> &str {
        self.path.as_str()
    }

    /// 获取分区深度
    pub fn get_depth(&self) -> usize {
        self.path.get_depth()
    }

    /// 检查是否为嵌套分区
    pub fn is_nested(&self) -> bool {
        self.path.get_depth() > 1
    }

    /// 获取摘要信息
    pub fn get_summary(&self) -> String {
        format!(
            "  Partition:\n    Path: {}\n    Depth: {}\n    Nested: {}\n",
            self.get_path(),
            self.get_depth(),
            self.is_nested()
        )
    }
}

/// 分区路径
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PartitionPath {
    path: PathBuf,
}

impl PartitionPath {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    pub fn as_str(&self) -> &str {
        self.path.to_str().unwrap_or("")
    }

    pub fn as_path(&self) -> &Path {
        &self.path
    }

    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // 基础验证
        if self.path.as_os_str().is_empty() {
            errors.push("Partition path cannot be empty".to_string());
            return Err(errors);
        }

        if !self.path.is_absolute() {
            errors.push(format!("Partition path must be absolute: {:?}", self.path));
        }

        let path_str = self.path.to_string_lossy();

        // 路径格式验证
        if path_str != "/" && path_str.ends_with('/') {
            errors.push("Partition path should not end with '/'".to_string());
        }

        if path_str.contains("//") {
            errors.push("Partition path should not contain empty components".to_string());
        }

        if path_str.contains("/./")
            || path_str.contains("/../")
            || path_str.starts_with("./")
            || path_str.starts_with("../")
        {
            errors.push("Partition path should not contain relative components".to_string());
        }

        // 组件验证
        for component in self.path.components() {
            if let std::path::Component::Normal(os_str) = component {
                let comp = os_str.to_string_lossy();

                if comp.is_empty() {
                    errors.push("Partition component cannot be empty".to_string());
                    continue;
                }

                if comp.len() > 255 {
                    errors.push(format!(
                        "Partition component too long ({} chars): {}",
                        comp.len(),
                        comp
                    ));
                }

                // 检查无效字符（适用于cgroups路径）
                if comp.contains(|c: char| {
                    c == '\0'
                        || c == '/'
                        || c == '\\'
                        || c == ':'
                        || c == '*'
                        || c == '?'
                        || c == '"'
                        || c == '<'
                        || c == '>'
                        || c == '|'
                }) {
                    errors.push(format!(
                        "Partition component contains invalid character: {}",
                        comp
                    ));
                }

                // cgroups 特定限制：不能以点开头（除了 "." 和 ".."）
                if comp.starts_with('.') && comp != "." && comp != ".." {
                    errors.push(format!(
                        "Partition component should not start with dot: {}",
                        comp
                    ));
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn get_depth(&self) -> usize {
        if self.path == Path::new("/") {
            return 0; // 根分区深度为0
        }

        self.path.components().count()
    }

    pub fn get_parent(&self) -> Option<PartitionPath> {
        self.path.parent().map(PartitionPath::new)
    }

    pub fn is_root(&self) -> bool {
        self.path == Path::new("/")
    }

    pub fn normalize(&self) -> Self {
        let mut normalized = PathBuf::new();

        for component in self.path.components() {
            match component {
                std::path::Component::Prefix(_) | std::path::Component::RootDir => {
                    normalized.push(component);
                }
                std::path::Component::CurDir => {
                    // 忽略当前目录组件
                }
                std::path::Component::ParentDir => {
                    if normalized.parent().is_some() {
                        normalized.pop();
                    }
                }
                std::path::Component::Normal(name) => {
                    normalized.push(name);
                }
            }
        }

        // 确保非空路径
        if normalized.as_os_str().is_empty() {
            normalized.push("/");
        }

        PartitionPath::new(normalized)
    }
}

// 序列化实现
impl Serialize for PartitionPath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for PartitionPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct PartitionPathVisitor;

        impl<'de> serde::de::Visitor<'de> for PartitionPathVisitor {
            type Value = PartitionPath;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a partition path string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(PartitionPath::new(value))
            }
        }

        deserializer.deserialize_str(PartitionPathVisitor)
    }
}
