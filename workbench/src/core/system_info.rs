use raw_cpuid::CpuId;
use sysinfo::System;

use crate::models::{CpuInfo, GpuInfo, MemoryInfo, OsInfo, StorageInfo, StorageType, SystemInfo};

pub struct SystemInfoCollector;

impl SystemInfoCollector {
    pub fn collect() -> SystemInfo {
        let mut sys = System::new_all();
        sys.refresh_all();

        SystemInfo {
            hostname: System::host_name().unwrap_or_else(|| "Unknown".to_string()),
            cpu: Self::collect_cpu_info(&sys),
            memory: Self::collect_memory_info(&sys),
            storage: Self::collect_storage_info(&sys),
            gpu: Self::collect_gpu_info(),
            os: Self::collect_os_info(),
        }
    }

    fn collect_cpu_info(sys: &System) -> CpuInfo {
        let cpuid = CpuId::new();

        let name = cpuid
            .get_processor_brand_string()
            .map(|b| b.as_str().trim().to_string())
            .unwrap_or_else(|| {
                sys.cpus()
                    .first()
                    .map(|c| c.brand().to_string())
                    .unwrap_or_else(|| "Unknown CPU".to_string())
            });

        let vendor = cpuid
            .get_vendor_info()
            .map(|v| v.as_str().to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        let cores = sys.physical_core_count().unwrap_or(1) as u32;
        let threads = sys.cpus().len() as u32;

        let base_frequency_mhz = sys
            .cpus()
            .first()
            .map(|c| c.frequency() as u32)
            .unwrap_or(0);

        let cache_l3_kb = cpuid
            .get_cache_parameters()
            .and_then(|mut caches| {
                caches.find(|c| c.level() == 3).map(|c| {
                    let ways = c.associativity();
                    let partitions = c.physical_line_partitions();
                    let line_size = c.coherency_line_size();
                    let sets = c.sets();
                    ((ways * partitions * line_size * sets) / 1024) as u32
                })
            });

        CpuInfo {
            name,
            vendor,
            cores,
            threads,
            base_frequency_mhz,
            max_frequency_mhz: None,
            cache_l3_kb,
        }
    }

    fn collect_memory_info(sys: &System) -> MemoryInfo {
        MemoryInfo {
            total_bytes: sys.total_memory(),
            speed_mhz: None,
            memory_type: None,
        }
    }

    fn collect_storage_info(sys: &System) -> Vec<StorageInfo> {
        sysinfo::Disks::new_with_refreshed_list()
            .iter()
            .map(|disk| {
                let name = disk.name().to_string_lossy().to_string();
                let device_type = Self::detect_storage_type(&name, disk);

                StorageInfo {
                    name: if name.is_empty() {
                        disk.mount_point().to_string_lossy().to_string()
                    } else {
                        name
                    },
                    device_type,
                    capacity_bytes: disk.total_space(),
                }
            })
            .collect()
    }

    fn detect_storage_type(name: &str, disk: &sysinfo::Disk) -> StorageType {
        let name_lower = name.to_lowercase();

        if name_lower.contains("nvme") {
            return StorageType::NVMe;
        }

        if name_lower.contains("ssd") {
            return StorageType::SSD;
        }

        if name_lower.contains("hdd") || name_lower.contains("hard") {
            return StorageType::HDD;
        }

        // Check by disk kind if available
        match disk.kind() {
            sysinfo::DiskKind::SSD => StorageType::SSD,
            sysinfo::DiskKind::HDD => StorageType::HDD,
            _ => StorageType::Unknown,
        }
    }

    fn collect_gpu_info() -> Option<GpuInfo> {
        // GPU detection is platform-specific and complex
        // For now, return None - will be implemented in Phase 5
        None
    }

    fn collect_os_info() -> OsInfo {
        OsInfo {
            name: System::name().unwrap_or_else(|| "Unknown".to_string()),
            version: System::os_version().unwrap_or_else(|| "Unknown".to_string()),
            build: System::kernel_version(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_system_info() {
        let info = SystemInfoCollector::collect();

        assert!(!info.hostname.is_empty());
        assert!(!info.cpu.name.is_empty());
        assert!(info.cpu.cores > 0);
        assert!(info.cpu.threads > 0);
        assert!(info.memory.total_bytes > 0);
    }
}
