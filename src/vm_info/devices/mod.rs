mod disk;
use disk::Disk;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Devices {
    pub disk: Vec<Disk>,
    // 可扩展其他设备（网卡、控制器等）
}
