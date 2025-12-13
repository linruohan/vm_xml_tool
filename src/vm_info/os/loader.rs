use serde::{Deserialize, Serialize};

// 可选的加载器标签引用一个固件blob，它由绝对路径指定，用于协助域创建过程。它用于Xen完全虚拟化域，以及为QEMU/KVM域设置QEMU BIOS文件路径。然后，从1.2.8开始，元素可能有两个可选的属性：readonly（可接受的值是yes和no），以反映映像应该是可写或只读的事实。第二个属性类型接受值rom和pflash。它告诉管理程序应该将文件映射到客户机内存中的哪个位置。例如，如果加载程序路径指向UEFI映像，类型应该是pflash。此外，一些固件可能实现安全启动特性。属性secure可用于告诉系统管理程序固件具有安全启动特性。它不能用于启用或禁用固件中的特性本身。因为魅惑。如果加载程序被标记为只读，那么UEFI假定将有一个可写的NVRAM可用。然而，在某些情况下，可能希望加载器在没有任何NVRAM的情况下运行，在关闭时丢弃任何配置更改。无状态标志（从8.6.0开始）可以用来控制这种行为，当设置为yes时，NVRAM将永远不会被创建。
//
//
// 当启用固件自动选择时，可以使用format属性告诉libvirt只考虑采用特定格式的固件构建。支持的值为raw和qcow2。自9.2.0起（仅限QEMU）
#[derive(Debug, Deserialize, Serialize)]
pub struct Loader {
    // 属性
    #[serde(rename = "@readonly", skip_serializing_if = "Option::is_none")]
    pub readonly: Option<String>, // "yes" or "no"

    #[serde(rename = "@secure", skip_serializing_if = "Option::is_none")]
    pub secure: Option<String>, // "yes" or "no"

    #[serde(rename = "@stateless", skip_serializing_if = "Option::is_none")]
    pub stateless: Option<String>, // "yes" or "no"

    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub loader_type: Option<String>, // e.g., "pflash"

    // 内容
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
