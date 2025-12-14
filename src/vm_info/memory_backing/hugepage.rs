use crate::MemoryUnit;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HugePages {
    #[serde(rename = "page")]
    pub pages: Vec<HugePage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HugePage {
    /// Size of hugepage in kiB (or unit specified)
    #[serde(rename = "@size")]
    pub size: u64,

    /// Unit for size attribute (default: kiB)
    #[serde(rename = "@unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<MemoryUnit>,

    /// NUMA nodeset for this hugepage size
    #[serde(rename = "@nodeset", skip_serializing_if = "Option::is_none")]
    pub nodeset: Option<String>,
}

impl HugePages {
    pub fn new(pages: Vec<HugePage>) -> Self {
        Self { pages }
    }
}

impl HugePage {
    pub fn new(size: u64, unit: Option<MemoryUnit>, nodeset: Option<String>) -> Self {
        Self {
            size,
            unit,
            nodeset,
        }
    }

    pub fn from_string(size: u64, unit_str: &str, nodeset_str: &str) -> Result<Self, String> {
        let unit = match unit_str {
            "B" => MemoryUnit::Bytes,
            "k" => MemoryUnit::KiB,
            "K" => MemoryUnit::K,
            "M" => MemoryUnit::MiB,
            "G" => MemoryUnit::GiB,
            "T" => MemoryUnit::TiB,
            _ => return Err(format!("Unknown unit: {}", unit_str)),
        };

        let nodeset = if nodeset_str.is_empty() {
            None
        } else {
            Some(NumaNodeSet::from_string(nodeset_str)?.to_string())
        };

        Ok(Self::new(size, Some(unit), nodeset))
    }
}
/// NUMA nodeset representation (e.g., "0-3,5")
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumaNodeSet {
    nodes: BTreeSet<u32>,
}
impl Display for NumaNodeSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.nodes.is_empty() {
            return Ok(());
        }

        let mut iter = self.nodes.iter().peekable();
        let mut first = true;

        while let Some(&start) = iter.next() {
            let mut end = start;

            // 查找连续的范围
            while let Some(&&next) = iter.peek() {
                if next == end + 1 {
                    end = next;
                    iter.next();
                } else {
                    break;
                }
            }

            if !first {
                write!(f, ",")?;
            }

            if start == end {
                write!(f, "{}", start)?;
            } else {
                write!(f, "{}-{}", start, end)?;
            }

            first = false;
        }

        Ok(())
    }
}

impl NumaNodeSet {
    pub fn new() -> Self {
        Self {
            nodes: BTreeSet::new(),
        }
    }

    pub fn from_string(s: &str) -> Result<Self, String> {
        let mut nodes = BTreeSet::new();

        for part in s.split(',') {
            if part.contains('-') {
                // Range like "0-3"
                let range_parts: Vec<&str> = part.split('-').collect();
                if range_parts.len() != 2 {
                    return Err(format!("Invalid range format: {}", part));
                }

                let start: u32 = range_parts[0]
                    .parse()
                    .map_err(|_| format!("Invalid start number: {}", range_parts[0]))?;
                let end: u32 = range_parts[1]
                    .parse()
                    .map_err(|_| format!("Invalid end number: {}", range_parts[1]))?;

                for node in start..=end {
                    nodes.insert(node);
                }
            } else {
                // Single node
                let node: u32 = part
                    .parse()
                    .map_err(|_| format!("Invalid node number: {}", part))?;
                nodes.insert(node);
            }
        }

        Ok(Self { nodes })
    }

    pub fn contains(&self, node: u32) -> bool {
        self.nodes.contains(&node)
    }

    pub fn add(&mut self, node: u32) {
        self.nodes.insert(node);
    }
}
