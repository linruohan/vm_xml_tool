use serde::{Deserialize, Serialize};
use std::fmt;

// 可选的memtune元素提供关于域的内存可调参数的详细信息。如果省略此选项，则默认使用操作系统提供的默认值。对于QEMU/KVM，这些参数作为一个整体应用于QEMU进程。因此，在计算它们时，需要将来宾RAM、来宾视频RAM和QEMU本身的一些内存开销加起来。最后一部分很难确定，所以需要猜测和尝试。对于每个可调项，可以在输入时指定数字所在的单元，使用与<memory>相同的值。为了向后兼容，输出总是KiB格式。*_limit的取值范围为0 ~ VIR_DOMAIN_MEMORY_PARAM_UNLIMITED。
// 主结构体
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Memtune {
    #[serde(rename = "hard_limit")]
    pub hard_limit: Limit,

    #[serde(rename = "soft_limit")]
    pub soft_limit: Limit,

    #[serde(rename = "swap_hard_limit")]
    pub swap_hard_limit: Limit,

    #[serde(rename = "min_guarantee")]
    pub min_guarantee: Limit,
}

// 限制结构体（包含值和单位）
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Limit {
    #[serde(rename = "@unit")]
    pub unit: String,

    #[serde(rename = "$value")]
    pub value: u64,
}

// 实现 Display trait 以便打印
impl fmt::Display for Limit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit)
    }
}

impl fmt::Display for Memtune {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Memory Tuning Settings:")?;
        writeln!(f, "  Hard Limit: {}", self.hard_limit)?;
        writeln!(f, "  Soft Limit: {}", self.soft_limit)?;
        writeln!(f, "  Swap Hard Limit: {}", self.swap_hard_limit)?;
        write!(f, "  Min Guarantee: {}", self.min_guarantee)
    }
}
