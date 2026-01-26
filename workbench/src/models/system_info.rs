use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub hostname: String,
    pub cpu: CpuInfo,
    pub memory: MemoryInfo,
    pub storage: Vec<StorageInfo>,
    pub gpu: Option<GpuInfo>,
    pub os: OsInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuInfo {
    pub name: String,
    pub vendor: String,
    pub cores: u32,
    pub threads: u32,
    pub base_frequency_mhz: u32,
    pub max_frequency_mhz: Option<u32>,
    pub cache_l3_kb: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    pub total_bytes: u64,
    pub speed_mhz: Option<u32>,
    pub memory_type: Option<String>,
}

impl MemoryInfo {
    pub fn total_gb(&self) -> f64 {
        self.total_bytes as f64 / (1024.0 * 1024.0 * 1024.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageInfo {
    pub name: String,
    pub device_type: StorageType,
    pub capacity_bytes: u64,
}

impl StorageInfo {
    pub fn capacity_gb(&self) -> f64 {
        self.capacity_bytes as f64 / (1024.0 * 1024.0 * 1024.0)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StorageType {
    NVMe,
    SSD,
    HDD,
    Unknown,
}

impl StorageType {
    pub fn label(&self) -> &'static str {
        match self {
            StorageType::NVMe => "NVMe",
            StorageType::SSD => "SSD",
            StorageType::HDD => "HDD",
            StorageType::Unknown => "Unknown",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    pub name: String,
    pub vendor: String,
    pub vram_bytes: Option<u64>,
    pub driver_version: Option<String>,
}

impl GpuInfo {
    pub fn vram_gb(&self) -> Option<f64> {
        self.vram_bytes.map(|b| b as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsInfo {
    pub name: String,
    pub version: String,
    pub build: Option<String>,
}
