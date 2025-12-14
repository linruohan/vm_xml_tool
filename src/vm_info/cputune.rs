use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Cputune {
    #[serde(rename = "vcpupin", default)]
    pub vcpu_pins: Vec<Vcpupin>,

    #[serde(rename = "emulatorpin", default)]
    pub emulator_pin: Option<Emulatorpin>,

    #[serde(rename = "iothreadpin", default)]
    pub iothread_pins: Vec<Iothreadpin>,

    #[serde(rename = "shares")]
    pub shares: Option<u32>,

    #[serde(rename = "period")]
    pub period: Option<u64>,

    #[serde(rename = "quota")]
    pub quota: Option<i64>,

    #[serde(rename = "global_period")]
    pub global_period: Option<u64>,

    #[serde(rename = "global_quota")]
    pub global_quota: Option<i64>,

    #[serde(rename = "emulator_period")]
    pub emulator_period: Option<u64>,

    #[serde(rename = "emulator_quota")]
    pub emulator_quota: Option<i64>,

    #[serde(rename = "iothread_period")]
    pub iothread_period: Option<u64>,

    #[serde(rename = "iothread_quota")]
    pub iothread_quota: Option<i64>,

    #[serde(rename = "vcpusched")]
    pub vcpu_sched: Option<Vcpusched>,

    #[serde(rename = "iothreadsched")]
    pub iothread_sched: Option<Iothreadsched>,

    #[serde(rename = "cachetune", default)]
    pub cache_tunes: Vec<Cachetune>,

    #[serde(rename = "memorytune", default)]
    pub memory_tunes: Vec<Memorytune>,
}

// vCPU将被固定到哪个主机的物理cpu 如果不指定该参数，且不指定元素vcpu的cpuset属性，则vcpu默认固定在所有物理cpu上。它包含两个必选属性，属性vcpu指定vcpu id，属性cpuset与元素vcpu的属性cpuset相同
#[derive(Debug, Deserialize, Serialize)]
pub struct Vcpupin {
    #[serde(rename = "@vcpu")]
    pub vcpu: String,
    #[serde(rename = "@cpuset")]
    pub cpuset: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Emulatorpin {
    #[serde(rename = "@cpuset")]
    pub cpuset: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Iothreadpin {
    #[serde(rename = "@iothread")]
    pub iothread: String,
    #[serde(rename = "@cpuset")]
    pub cpuset: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Vcpusched {
    #[serde(rename = "@vcpus")]
    pub vcpus: String,
    #[serde(rename = "@scheduler")]
    pub scheduler: String,
    #[serde(rename = "@priority")]
    pub priority: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Iothreadsched {
    #[serde(rename = "@iothreads")]
    pub iothreads: String,
    #[serde(rename = "@scheduler")]
    pub scheduler: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Cachetune {
    #[serde(rename = "@vcpus")]
    pub vcpus: String,
    #[serde(rename = "cache", default)]
    pub caches: Vec<Cache>,
    #[serde(rename = "monitor", default)]
    pub monitors: Vec<Monitor>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Cache {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@level")]
    pub level: String,
    #[serde(rename = "@type")]
    pub cache_type: String,
    #[serde(rename = "@size")]
    pub size: String,
    #[serde(rename = "@unit")]
    pub unit: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Monitor {
    #[serde(rename = "@level")]
    pub level: String,
    #[serde(rename = "@vcpus")]
    pub vcpus: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Memorytune {
    #[serde(rename = "@vcpus")]
    pub vcpus: String,
    #[serde(rename = "node", default)]
    pub nodes: Vec<Node>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Node {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@bandwidth")]
    pub bandwidth: String,
}