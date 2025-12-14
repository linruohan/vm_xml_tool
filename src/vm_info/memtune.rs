use crate::MemoryValue;
use serde::{Deserialize, Serialize};

// 可选的memtune元素提供关于域的内存可调参数的详细信息。如果省略此选项，则默认使用操作系统提供的默认值。对于QEMU/KVM，这些参数作为一个整体应用于QEMU进程。因此，在计算它们时，需要将来宾RAM、来宾视频RAM和QEMU本身的一些内存开销加起来。最后一部分很难确定，所以需要猜测和尝试。对于每个可调项，可以在输入时指定数字所在的单元，使用与<memory>相同的值。为了向后兼容，输出总是KiB格式。*_limit的取值范围为0 ~ VIR_DOMAIN_MEMORY_PARAM_UNLIMITED。
// 主结构体
/// 内存调优配置
#[derive(Debug, Serialize, Deserialize, PartialEq, Default, Clone)]
pub struct MemTune {
    #[serde(rename = "hard_limit", skip_serializing_if = "Option::is_none")]
    pub hard_limit: Option<MemoryValue>,

    #[serde(rename = "soft_limit", skip_serializing_if = "Option::is_none")]
    pub soft_limit: Option<MemoryValue>,

    #[serde(rename = "swap_hard_limit", skip_serializing_if = "Option::is_none")]
    pub swap_hard_limit: Option<MemoryValue>,

    #[serde(rename = "min_guarantee", skip_serializing_if = "Option::is_none")]
    pub min_guarantee: Option<MemoryValue>,
}
