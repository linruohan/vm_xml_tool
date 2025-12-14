use serde::{Deserialize, Serialize};

// 在引导时为客户机分配的最大内存。内存分配包括在启动时指定或稍后热插拔时指定的可能的附加内存设备。
// 此值的单位由可选属性unit决定，默认为“KiB”（kibibytes， 210或1024字节的块）。有效的单位是字节的“b”或“bytes”，千字节的“KB”（103或1000字节），千兆字节的“k”或“KiB”（1024字节），兆字节的“MB”（106或1,000,000字节），兆字节的“M”或“MiB”（220或1,048,576字节），千兆字节的“GB”（109或1,000,000,000字节），千兆字节的“G”或“GiB”（230或1,073,741,824字节），TB的“TB”（1012或1,000,000,000,000字节），或“T”或“TiB”的tebibytes（240或1,099,511,627,776字节）。但是，libvirt会将该值四舍五入到最接近的千字节，并可能进一步四舍五入到hypervisor支持的粒度。一些管理程序还强制执行最小值，例如4000KiB。如果为客户机配置了NUMA（请参阅CPU模型和拓扑），则可以省略内存元素。在崩溃的情况下，可以使用可选属性dumpCore来控制是否应该将来宾内存包含在生成的coredump中（值为“on”、“off”）。unit从0.9.11，dumpCore从0.10.2（仅QEMU）
#[derive(Debug, Deserialize, Serialize)]
pub struct Memory {
    #[serde(rename = "@unit")]
    pub unit: String,
    #[serde(rename = "$text")]
    pub value: u64,
}
// 运行时最大内存分配。 <memory>元素或NUMA单元大小配置指定的初始内存可以通过将内存热插拔到该元素指定的限制来增加。
// unit属性的行为与<memory>相同。
// slots属性指定可用于向来宾添加内存的插槽数量。边界是特定于管理程序的
// 请注意，由于通过内存热插拔添加的内存块的对齐，此元素指定的完整大小分配可能无法实现
#[derive(Debug, Deserialize, Serialize)]
pub struct MaxMemory {
    #[serde(rename = "@slots")]
    pub slots: u32,
    #[serde(rename = "@unit")]
    pub unit: String,
    #[serde(rename = "$text")]
    pub value: u64,
}

// 为来宾分配的实际内存。这个值可以小于最大分配值，以便动态地增加来宾内存。如果省略，则默认为与内存元素相同的值。unit属性的行为与内存相同。
#[derive(Debug, Deserialize, Serialize)]
pub struct CurrentMemory {
    #[serde(rename = "@unit")]
    pub unit: String,
    #[serde(rename = "$text")]
    pub value: u64,
}
