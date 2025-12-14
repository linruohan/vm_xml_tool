use quick_xml::se::to_string;
use serde::{Deserialize, Serialize};

mod hugepage;
use hugepage::{HugePage, HugePages};
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct EmptyElement;
#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryBacking {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hugepages: Option<HugePages>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nosharepages: Option<EmptyElement>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<EmptyElement>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<MemorySource>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<Access>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation: Option<MemoryAllocation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub discard: Option<EmptyElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemorySourceType {
    #[serde(rename = "file")]
    File,
    #[serde(rename = "anonymous")]
    Anonymous,
    #[serde(rename = "memfd")]
    Memfd,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySource {
    #[serde(rename = "@type")]
    pub source_type: MemorySourceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Access {
    #[serde(rename = "@mode")]
    pub mode: MemoryAccessMode, // "shared" or "private"
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryAccessMode {
    #[serde(rename = "shared")]
    Shared,
    #[serde(rename = "private")]
    Private,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryAllocation {
    #[serde(rename = "@mode")]
    pub mode: MemoryAllocationMode, // "immediate" or "ondemand"
    #[serde(rename = "@threads", skip_serializing_if = "Option::is_none")]
    pub threads: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryAllocationMode {
    #[serde(rename = "immediate")]
    Immediate,
    #[serde(rename = "ondemand")]
    Ondemand,
}

// Builder pattern for easy construction
impl MemoryBacking {
    pub fn new() -> Self {
        Self {
            hugepages: None,
            nosharepages: None,
            locked: None,
            source: None,
            access: None,
            allocation: None,
            discard: None,
        }
    }

    pub fn with_hugepages(mut self, hugepages: HugePages) -> Self {
        self.hugepages = Some(hugepages);
        self
    }

    pub fn with_nosharepages(mut self) -> Self {
        self.nosharepages = Some(EmptyElement);
        self
    }

    pub fn with_locked(mut self) -> Self {
        self.locked = Some(EmptyElement);
        self
    }

    pub fn with_source(mut self, source_type: MemorySourceType) -> Self {
        self.source = Some(MemorySource { source_type });
        self
    }

    pub fn with_access(mut self, mode: Access) -> Self {
        self.access = Some(mode);
        self
    }

    pub fn with_allocation(mut self, mode: MemoryAllocationMode, threads: Option<u32>) -> Self {
        self.allocation = Some(MemoryAllocation { mode, threads });
        self
    }

    pub fn with_discard(mut self) -> Self {
        self.discard = Some(EmptyElement);
        self
    }
}

// 使用示例
fn run() -> Result<(), String> {
    // 创建示例配置
    let memory_backing = MemoryBacking::new()
        .with_hugepages(HugePages::new(vec![
            HugePage::from_string(1, "G", "0-3,5")?,
            HugePage::from_string(2, "M", "4")?,
        ]))
        .with_nosharepages()
        .with_locked()
        .with_source(MemorySourceType::Anonymous)
        .with_access(Access {
            mode: MemoryAccessMode::Shared,
        })
        .with_allocation(MemoryAllocationMode::Immediate, Some(8))
        .with_discard();

    // 输出XML
    println!("{:?}", to_string(&memory_backing));

    // 输出JSON（用于调试）
    println!("\nJSON representation:");
    println!("{}", serde_json::to_string_pretty(&memory_backing).unwrap());

    Ok(())
}
