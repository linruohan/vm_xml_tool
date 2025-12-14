mod fwcfg;
mod smbios;

use fwcfg::{FwcfgEntry, FwcfgSysinfo};
use serde::de::{MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use smbios::SmbiosSysinfo;
use smbios::{BaseBoardInfo, BiosInfo, ChassisInfo, OemStringsInfo, SystemInfo};
use std::fmt;

// sysinfo 解析有问题，暂时不能用
#[derive(Debug, Deserialize, Serialize)]
pub struct Sysinfo {
    #[serde(rename = "@type")]
    pub sysinfo_type: String, // "smbios" 或 "fwcfg"

    // 根据类型选择不同的内容
    #[serde(flatten)]
    pub content: SysinfoContent,
}

fn deserialize_entries<'de, D>(deserializer: D) -> Result<Vec<FwcfgEntry>, D::Error>
                                                where
                                                    D: Deserializer<'de>,
{
    struct EntryVisitor;

    impl<'de> Visitor<'de> for EntryVisitor {
        type Value = Vec<FwcfgEntry>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a single entry or a sequence of entries")
        }

        // 处理数组情况
        fn visit_seq<A>(self, seq: A) -> Result<Vec<FwcfgEntry>, A::Error>
                                      where
                                          A: SeqAccess<'de>,
        {
            let mut entries = Vec::new();
            let mut seq = seq;

            while let Some(entry) = seq.next_element()? {
                entries.push(entry);
            }

            Ok(entries)
        }

        // 处理单个元素情况
        fn visit_map<M>(self, map: M) -> Result<Vec<FwcfgEntry>, M::Error>
                                      where
                                          M: MapAccess<'de>,
        {
            let entry = FwcfgEntry::deserialize(serde::de::value::MapAccessDeserializer::new(map))?;
            Ok(vec![entry])
        }
    }

    deserializer.deserialize_any(EntryVisitor)
}

// 中间结构体
#[derive(Debug, Deserialize)]
struct SysinfoIntermediate {
    #[serde(rename = "@type")]
    sysinfo_type: String,
    bios: Option<BiosInfo>,
    system: Option<SystemInfo>,
    #[serde(rename = "baseBoard")]
    base_board: Option<BaseBoardInfo>,
    chassis: Option<ChassisInfo>,
    #[serde(rename = "oemStrings")]
    oem_strings: Option<OemStringsInfo>,
    #[serde(rename = "entry", deserialize_with = "deserialize_entries", default)]
    entries: Vec<FwcfgEntry>,
}
#[derive(Debug)]
pub struct SysinfoContent {
    // 内部使用枚举
    inner: SysinfoContentEnum,
}

// 使用枚举处理不同类型的 sysinfo 内容
#[derive(Debug, PartialEq)]
pub enum SysinfoContentEnum {
    Smbios(SmbiosSysinfo),
    Fwcfg(FwcfgSysinfo),
}
impl From<SysinfoIntermediate> for SysinfoContent {
    fn from(intermediate: SysinfoIntermediate) -> Self {
        let inner = match intermediate.sysinfo_type.as_str() {
            "smbios" => {
                let smbios = SmbiosSysinfo {
                    bios: intermediate.bios.into_iter().next(),
                    system: intermediate.system.into_iter().next(),
                    base_board: intermediate.base_board.into_iter().next(),
                    chassis: intermediate.chassis.into_iter().next(),
                    oem_strings: intermediate.oem_strings.into_iter().next(),
                };
                SysinfoContentEnum::Smbios(smbios)
            }
            "fwcfg" => {
                let fwcfg = FwcfgSysinfo {
                    entries: intermediate.entries,
                };
                SysinfoContentEnum::Fwcfg(fwcfg)
            }
            _ => panic!("Unknown sysinfo type: {}", intermediate.sysinfo_type),
        };
        SysinfoContent { inner }
    }
}
impl<'de> Deserialize<'de> for SysinfoContent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                                       where
                                           D: Deserializer<'de>,
    {
        let intermediate = SysinfoIntermediate::deserialize(deserializer)?;
        Ok(SysinfoContent::from(intermediate))
    }
}
impl Serialize for SysinfoContent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                                          where
                                              S: serde::Serializer,
    {
        match &self.inner {
            SysinfoContentEnum::Smbios(smbios) => smbios.serialize(serializer),
            SysinfoContentEnum::Fwcfg(fwcfg) => fwcfg.serialize(serializer),
        }
    }
}
