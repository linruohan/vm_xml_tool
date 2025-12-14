use crate::{MemoryUnit, MemoryValue};
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

impl MemTune {
    /// 创建新的 MemTune 配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置硬限制
    pub fn with_hard_limit(mut self, value: u64, unit: MemoryUnit) -> Self {
        self.hard_limit = Some(MemoryValue::new(value, unit));
        self
    }

    /// 设置软限制
    pub fn with_soft_limit(mut self, value: u64, unit: MemoryUnit) -> Self {
        self.soft_limit = Some(MemoryValue::new(value, unit));
        self
    }

    /// 设置交换硬限制
    pub fn with_swap_hard_limit(mut self, value: u64, unit: MemoryUnit) -> Self {
        self.swap_hard_limit = Some(MemoryValue::new(value, unit));
        self
    }

    /// 设置最小保证内存
    pub fn with_min_guarantee(mut self, value: u64, unit: MemoryUnit) -> Self {
        self.min_guarantee = Some(MemoryValue::new(value, unit));
        self
    }

    /// 验证配置的有效性
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // 验证 swap_hard_limit > hard_limit
        if let (Some(hard), Some(swap)) = (&self.hard_limit, &self.swap_hard_limit) {
            let hard_kib = hard.to_kib();
            let swap_kib = swap.to_kib();

            if swap_kib <= hard_kib {
                errors.push(format!(
                    "swap_hard_limit ({}) must be greater than hard_limit ({})",
                    swap.to_human_readable(),
                    hard.to_human_readable()
                ));
            }
        }

        // 验证 soft_limit <= hard_limit
        if let (Some(hard), Some(soft)) = (&self.hard_limit, &self.soft_limit) {
            let hard_kib = hard.to_kib();
            let soft_kib = soft.to_kib();

            if soft_kib > hard_kib {
                errors.push(format!(
                    "soft_limit ({}) cannot exceed hard_limit ({})",
                    soft.to_human_readable(),
                    hard.to_human_readable()
                ));
            }
        }

        // 验证 min_guarantee <= hard_limit
        if let (Some(hard), Some(min)) = (&self.hard_limit, &self.min_guarantee) {
            let hard_kib = hard.to_kib();
            let min_kib = min.to_kib();

            if min_kib > hard_kib {
                errors.push(format!(
                    "min_guarantee ({}) cannot exceed hard_limit ({})",
                    min.to_human_readable(),
                    hard.to_human_readable()
                ));
            }
        }

        // 验证值范围
        let check_positive = |name: &str, value: u64| {
            if value == 0 {
                Some(format!("{} cannot be 0", name))
            } else {
                None
            }
        };

        // 检查所有存在的限制
        if let Some(hl) = &self.hard_limit
            && let Some(err) = check_positive("hard_limit", hl.value)
        {
            errors.push(err);
        }

        if let Some(sl) = &self.soft_limit
            && let Some(err) = check_positive("soft_limit", sl.value)
        {
            errors.push(err);
        }

        if let Some(swl) = &self.swap_hard_limit
            && let Some(err) = check_positive("swap_hard_limit", swl.value)
        {
            errors.push(err);
        }

        if let Some(mg) = &self.min_guarantee
            && let Some(err) = check_positive("min_guarantee", mg.value)
        {
            errors.push(err);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// 获取所有限制的 KiB 值
    pub fn get_limits_in_kib(&self) -> MemTuneKiB {
        MemTuneKiB {
            hard_limit: self.hard_limit.as_ref().map(|v| v.to_kib()),
            soft_limit: self.soft_limit.as_ref().map(|v| v.to_kib()),
            swap_hard_limit: self.swap_hard_limit.as_ref().map(|v| v.to_kib()),
            min_guarantee: self.min_guarantee.as_ref().map(|v| v.to_kib()),
        }
    }
}

/// 以 KiB 为单位的内存调优配置
#[derive(Debug, Default)]
pub struct MemTuneKiB {
    pub hard_limit: Option<u64>,
    pub soft_limit: Option<u64>,
    pub swap_hard_limit: Option<u64>,
    pub min_guarantee: Option<u64>,
}
