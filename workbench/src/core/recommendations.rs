//! Post-benchmark recommendations engine.
//!
//! Analyzes benchmark results and generates optimization recommendations.

use crate::models::{BenchmarkRun, StorageType};

/// Category of recommendation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecommendationCategory {
    /// Software/Windows settings optimization
    Software,
    /// Hardware upgrade suggestion
    Hardware,
}

impl RecommendationCategory {
    /// Get display label
    pub fn label(&self) -> &'static str {
        match self {
            RecommendationCategory::Software => "Software",
            RecommendationCategory::Hardware => "Hardware",
        }
    }
}

/// Priority level for recommendations
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RecommendationPriority {
    /// High priority - significant improvement expected
    High,
    /// Medium priority - moderate improvement expected
    Medium,
    /// Low priority - minor improvement expected
    Low,
}

impl RecommendationPriority {
    /// Get display label
    pub fn label(&self) -> &'static str {
        match self {
            RecommendationPriority::High => "High",
            RecommendationPriority::Medium => "Medium",
            RecommendationPriority::Low => "Low",
        }
    }
}

/// Device type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    /// Desktop workstation
    Desktop,
    /// Laptop computer
    Laptop,
    /// Virtual Desktop Infrastructure
    VDI,
    /// Unknown or unclassified
    Unknown,
}

impl DeviceType {
    /// Get display label
    pub fn label(&self) -> &'static str {
        match self {
            DeviceType::Desktop => "Desktop",
            DeviceType::Laptop => "Laptop",
            DeviceType::VDI => "VDI",
            DeviceType::Unknown => "Unknown",
        }
    }
}

/// A single optimization recommendation
#[derive(Debug, Clone)]
pub struct Recommendation {
    /// Unique identifier
    pub id: String,
    /// Short title
    pub title: String,
    /// Detailed description of the issue
    pub description: String,
    /// Category (Software or Hardware)
    pub category: RecommendationCategory,
    /// Priority level
    pub priority: RecommendationPriority,
    /// Expected improvement (e.g., "10-20% faster file ops")
    pub expected_improvement: String,
    /// Step-by-step instructions to apply
    pub how_to_apply: Vec<String>,
    /// Tests that would benefit from this recommendation
    pub affected_tests: Vec<String>,
}

/// Percentile rank for a specific test
#[derive(Debug, Clone)]
pub struct PercentileRank {
    /// Test identifier
    pub test_id: String,
    /// Percentile rank (0-100, where 100 is best)
    pub percentile: f32,
}

/// Complete recommendations report
#[derive(Debug, Clone)]
pub struct RecommendationsReport {
    /// Detected device type
    pub device_type: DeviceType,
    /// Overall percentile across all tests (if community data available)
    pub overall_percentile: Option<f32>,
    /// List of recommendations sorted by priority
    pub recommendations: Vec<Recommendation>,
}

/// Engine for generating recommendations based on benchmark results
pub struct RecommendationEngine;

impl RecommendationEngine {
    /// Analyze benchmark results and generate recommendations
    ///
    /// # Arguments
    /// * `run` - The completed benchmark run
    /// * `percentile_ranks` - Optional percentile ranks from community comparison
    pub fn analyze(
        run: &BenchmarkRun,
        percentile_ranks: Option<&[PercentileRank]>,
    ) -> RecommendationsReport {
        let mut recommendations = Vec::new();

        // Detect device type
        let device_type = Self::detect_device_type(run);

        // Analyze storage performance
        Self::analyze_storage(run, percentile_ranks, &mut recommendations);

        // Analyze CPU performance
        Self::analyze_cpu(run, percentile_ranks, &mut recommendations);

        // Analyze memory performance
        Self::analyze_memory(run, percentile_ranks, &mut recommendations);

        // Analyze Windows-specific tests
        Self::analyze_windows_performance(run, &mut recommendations);

        // Add general software recommendations
        Self::add_general_recommendations(run, &mut recommendations);

        // Sort by priority (High first, then Medium, then Low)
        recommendations.sort_by(|a, b| a.priority.cmp(&b.priority));

        // Calculate overall percentile if ranks provided
        let overall_percentile = percentile_ranks.map(|ranks| {
            if ranks.is_empty() {
                50.0
            } else {
                ranks.iter().map(|r| r.percentile).sum::<f32>() / ranks.len() as f32
            }
        });

        RecommendationsReport {
            device_type,
            overall_percentile,
            recommendations,
        }
    }

    fn detect_device_type(run: &BenchmarkRun) -> DeviceType {
        let hostname = run.machine_name.to_lowercase();

        // Check for VDI indicators
        if hostname.contains("vdi")
            || hostname.contains("virtual")
            || hostname.contains("citrix")
            || hostname.contains("vmware")
        {
            return DeviceType::VDI;
        }

        // Check system info for laptop indicators
        // Laptops typically have lower TDP CPUs with specific naming
        let cpu_name = run.system_info.cpu.name.to_lowercase();
        if cpu_name.contains(" u ") // Intel U-series (laptop)
            || cpu_name.ends_with("u")  // e.g., i7-1265U
            || cpu_name.contains(" p ") // Intel P-series (laptop)
            || cpu_name.ends_with("p")  // e.g., Intel P-series suffix
            || cpu_name.contains("mobile")
            || cpu_name.contains("laptop")
        {
            return DeviceType::Laptop;
        }

        // If high core count and desktop-class CPU naming, likely desktop
        if run.system_info.cpu.cores >= 6
            && (cpu_name.contains(" k ") // Intel K-series (desktop)
                || cpu_name.contains("-k")
                || cpu_name.contains(" x ") // Intel X-series
                || cpu_name.contains("-x")
                || cpu_name.contains("ryzen 9")
                || cpu_name.contains("ryzen 7"))
        {
            return DeviceType::Desktop;
        }

        DeviceType::Unknown
    }

    fn analyze_storage(
        run: &BenchmarkRun,
        percentile_ranks: Option<&[PercentileRank]>,
        recommendations: &mut Vec<Recommendation>,
    ) {
        // Check storage type
        let has_ssd = run.system_info.storage.iter().any(|s| {
            matches!(s.device_type, StorageType::SSD | StorageType::NVMe)
        });

        let has_only_hdd = !has_ssd && run.system_info.storage.iter().any(|s| {
            matches!(s.device_type, StorageType::HDD)
        });

        // Find storage-related test results
        let random_read = run.results.project_operations.iter()
            .find(|r| r.test_id == "random_read");
        let file_enum = run.results.project_operations.iter()
            .find(|r| r.test_id == "file_enumeration");

        // HDD upgrade recommendation
        if has_only_hdd {
            recommendations.push(Recommendation {
                id: "upgrade_to_ssd".to_string(),
                title: "Upgrade to SSD".to_string(),
                description: "Your system is using a traditional hard drive (HDD). \
                             SSDs provide dramatically faster random access times \
                             and are the single most impactful upgrade for developer workstations."
                    .to_string(),
                category: RecommendationCategory::Hardware,
                priority: RecommendationPriority::High,
                expected_improvement: "10-50x faster file operations, 2-5x faster builds".to_string(),
                how_to_apply: vec![
                    "Consider a SATA SSD for budget builds or NVMe for best performance".to_string(),
                    "500GB-1TB is recommended for development work".to_string(),
                    "Clone existing drive or fresh install Windows".to_string(),
                    "Enable TRIM after installation for optimal SSD performance".to_string(),
                ],
                affected_tests: vec![
                    "Random Read".to_string(),
                    "File Enumeration".to_string(),
                    "Directory Traversal".to_string(),
                    "Large File Read".to_string(),
                    "Storage Latency".to_string(),
                ],
            });
        }

        // Check for slow storage performance even with SSD
        if let Some(random_read_result) = random_read {
            // P99 latency > 5ms is concerning for SSD
            if random_read_result.value > 5.0 && has_ssd {
                // Check percentile if available
                let is_slow = percentile_ranks
                    .and_then(|ranks| ranks.iter().find(|r| r.test_id == "random_read"))
                    .map(|r| r.percentile < 25.0)
                    .unwrap_or(true);

                if is_slow {
                    recommendations.push(Recommendation {
                        id: "optimize_ssd".to_string(),
                        title: "Optimize SSD Performance".to_string(),
                        description: "Your SSD random read latency is higher than expected. \
                                     This could be due to drive wear, firmware, or system settings."
                            .to_string(),
                        category: RecommendationCategory::Software,
                        priority: RecommendationPriority::Medium,
                        expected_improvement: "20-40% faster random file access".to_string(),
                        how_to_apply: vec![
                            "Verify TRIM is enabled: Run 'fsutil behavior query DisableDeleteNotify'".to_string(),
                            "Update SSD firmware from manufacturer website".to_string(),
                            "Ensure AHCI mode is enabled in BIOS (not IDE)".to_string(),
                            "Check drive health with manufacturer tools (e.g., Samsung Magician, Crucial Storage Executive)".to_string(),
                        ],
                        affected_tests: vec![
                            "Random Read".to_string(),
                            "Storage Latency".to_string(),
                        ],
                    });
                }
            }
        }

        // Check file enumeration performance
        if let Some(file_enum_result) = file_enum {
            // Less than 50,000 files/sec is slow for modern systems
            if file_enum_result.value < 50000.0 {
                recommendations.push(Recommendation {
                    id: "optimize_file_system".to_string(),
                    title: "Optimize File System Settings".to_string(),
                    description: "File enumeration performance is below optimal. \
                                 Windows NTFS settings may be limiting performance."
                        .to_string(),
                    category: RecommendationCategory::Software,
                    priority: RecommendationPriority::Medium,
                    expected_improvement: "10-30% faster directory listings".to_string(),
                    how_to_apply: vec![
                        "Disable 8.3 filename creation (Admin CMD): fsutil behavior set disable8dot3 1".to_string(),
                        "Disable last access time updates: fsutil behavior set disablelastaccess 1".to_string(),
                        "Consider excluding development folders from Windows Defender real-time scan".to_string(),
                    ],
                    affected_tests: vec![
                        "File Enumeration".to_string(),
                        "Directory Traversal".to_string(),
                        "Metadata Operations".to_string(),
                    ],
                });
            }
        }
    }

    fn analyze_cpu(
        run: &BenchmarkRun,
        percentile_ranks: Option<&[PercentileRank]>,
        recommendations: &mut Vec<Recommendation>,
    ) {
        let single_thread = run.results.build_performance.iter()
            .find(|r| r.test_id == "single_thread");
        let multi_thread = run.results.build_performance.iter()
            .find(|r| r.test_id == "multi_thread");

        // Check if all CPU tests are in bottom 25%
        let cpu_tests = ["single_thread", "multi_thread", "mixed_workload"];
        let all_cpu_slow = percentile_ranks
            .map(|ranks| {
                let cpu_ranks: Vec<_> = ranks
                    .iter()
                    .filter(|r| cpu_tests.contains(&r.test_id.as_str()))
                    .collect();
                !cpu_ranks.is_empty() && cpu_ranks.iter().all(|r| r.percentile < 25.0)
            })
            .unwrap_or(false);

        if all_cpu_slow {
            recommendations.push(Recommendation {
                id: "cpu_upgrade".to_string(),
                title: "Consider CPU Upgrade".to_string(),
                description: "Your CPU performance ranks in the bottom 25% across all tests. \
                             A CPU upgrade would significantly improve build times and responsiveness."
                    .to_string(),
                category: RecommendationCategory::Hardware,
                priority: RecommendationPriority::Medium,
                expected_improvement: "30-100% faster builds depending on upgrade".to_string(),
                how_to_apply: vec![
                    "Modern CPUs with 8+ cores recommended for development".to_string(),
                    "Consider AMD Ryzen 7/9 or Intel Core i7/i9 (12th gen or newer)".to_string(),
                    "Ensure motherboard and power supply support the upgrade".to_string(),
                ],
                affected_tests: vec![
                    "Single-Thread CPU".to_string(),
                    "Multi-Thread CPU".to_string(),
                    "Mixed Workload".to_string(),
                    "Native Compiler".to_string(),
                ],
            });
        }

        // Check for thermal throttling indicators
        if let (Some(single), Some(multi)) = (single_thread, multi_thread) {
            let expected_multi = single.value * run.system_info.cpu.threads as f64 * 0.7; // 70% scaling expected
            if multi.value < expected_multi * 0.5 {
                recommendations.push(Recommendation {
                    id: "check_thermal".to_string(),
                    title: "Check CPU Cooling".to_string(),
                    description: "Multi-threaded performance is significantly lower than expected \
                                 based on single-threaded results. This may indicate thermal throttling."
                        .to_string(),
                    category: RecommendationCategory::Hardware,
                    priority: RecommendationPriority::High,
                    expected_improvement: "20-50% faster multi-threaded performance".to_string(),
                    how_to_apply: vec![
                        "Check CPU temperatures under load (use HWiNFO or similar)".to_string(),
                        "Clean dust from CPU cooler and case fans".to_string(),
                        "Reapply thermal paste if temperatures exceed 90°C under load".to_string(),
                        "Consider upgrading CPU cooler if using stock cooler".to_string(),
                    ],
                    affected_tests: vec![
                        "Multi-Thread CPU".to_string(),
                        "Mixed Workload".to_string(),
                    ],
                });
            }
        }
    }

    fn analyze_memory(
        run: &BenchmarkRun,
        percentile_ranks: Option<&[PercentileRank]>,
        recommendations: &mut Vec<Recommendation>,
    ) {
        let memory_bandwidth = run.results.responsiveness.iter()
            .find(|r| r.test_id == "memory_bandwidth");
        let memory_latency = run.results.responsiveness.iter()
            .find(|r| r.test_id == "memory_latency");

        // Check RAM amount
        let total_ram_gb = run.system_info.memory.total_gb();
        if total_ram_gb < 16.0 {
            recommendations.push(Recommendation {
                id: "add_ram".to_string(),
                title: "Add More RAM".to_string(),
                description: format!(
                    "Your system has {:.0}GB of RAM. Modern development workflows \
                     (IDEs, Docker, browsers, build tools) benefit greatly from 16GB or more.",
                    total_ram_gb
                ),
                category: RecommendationCategory::Hardware,
                priority: if total_ram_gb < 8.0 {
                    RecommendationPriority::High
                } else {
                    RecommendationPriority::Medium
                },
                expected_improvement: "Reduced swapping, faster context switching".to_string(),
                how_to_apply: vec![
                    "Check current RAM configuration with Task Manager > Performance > Memory".to_string(),
                    "Verify motherboard supports additional RAM slots or higher capacity".to_string(),
                    "Match existing RAM speed and timings for best compatibility".to_string(),
                    "16GB recommended minimum, 32GB for heavy Docker/VM usage".to_string(),
                ],
                affected_tests: vec![
                    "Memory Bandwidth".to_string(),
                    "Process Spawn".to_string(),
                    "Application Launch".to_string(),
                ],
            });
        }

        // Check memory bandwidth against percentile
        if let Some(bw_result) = memory_bandwidth {
            let is_slow = percentile_ranks
                .and_then(|ranks| ranks.iter().find(|r| r.test_id == "memory_bandwidth"))
                .map(|r| r.percentile < 25.0)
                .unwrap_or(false);

            if is_slow && bw_result.value < 20.0 {
                // Less than 20 GB/s is quite slow for modern systems
                recommendations.push(Recommendation {
                    id: "optimize_ram".to_string(),
                    title: "Optimize RAM Configuration".to_string(),
                    description: "Memory bandwidth is below expected levels. \
                                 This could be due to single-channel configuration or slow RAM."
                        .to_string(),
                    category: RecommendationCategory::Hardware,
                    priority: RecommendationPriority::Medium,
                    expected_improvement: "20-50% faster memory operations".to_string(),
                    how_to_apply: vec![
                        "Ensure RAM is installed in dual-channel configuration (matching slots)".to_string(),
                        "Enable XMP/DOCP profile in BIOS for rated RAM speed".to_string(),
                        "Check RAM speed in Task Manager > Performance > Memory".to_string(),
                    ],
                    affected_tests: vec![
                        "Memory Bandwidth".to_string(),
                        "Memory Latency".to_string(),
                    ],
                });
            }
        }
    }

    fn analyze_windows_performance(
        run: &BenchmarkRun,
        recommendations: &mut Vec<Recommendation>,
    ) {
        // Check Defender Impact
        let defender_impact = run.results.project_operations.iter()
            .find(|r| r.test_id == "defender_impact");

        if let Some(defender_result) = defender_impact {
            if defender_result.value > 30.0 {
                // More than 30% overhead from Defender
                recommendations.push(Recommendation {
                    id: "configure_defender".to_string(),
                    title: "Configure Windows Defender Exclusions".to_string(),
                    description: format!(
                        "Windows Defender real-time scanning adds {:.0}% overhead to file operations. \
                         Adding exclusions for development folders can significantly improve performance.",
                        defender_result.value
                    ),
                    category: RecommendationCategory::Software,
                    priority: if defender_result.value > 50.0 {
                        RecommendationPriority::High
                    } else {
                        RecommendationPriority::Medium
                    },
                    expected_improvement: format!("Up to {:.0}% faster file operations", defender_result.value),
                    how_to_apply: vec![
                        "Open Windows Security > Virus & threat protection > Manage settings".to_string(),
                        "Scroll to Exclusions > Add or remove exclusions".to_string(),
                        "Add folder exclusions for: source code directories, node_modules, .git folders".to_string(),
                        "Add process exclusions for: devenv.exe, code.exe, node.exe, cargo.exe".to_string(),
                        "Note: Only exclude trusted development folders".to_string(),
                    ],
                    affected_tests: vec![
                        "File Enumeration".to_string(),
                        "Directory Traversal".to_string(),
                        "Native Compiler".to_string(),
                        "Archive Operations".to_string(),
                    ],
                });
            }
        }

        // Check PowerShell performance
        let powershell = run.results.build_performance.iter()
            .find(|r| r.test_id == "powershell");

        if let Some(ps_result) = powershell {
            if ps_result.value > 500.0 {
                // More than 500ms average is slow
                recommendations.push(Recommendation {
                    id: "optimize_powershell".to_string(),
                    title: "Optimize PowerShell Startup".to_string(),
                    description: "PowerShell script execution is slower than expected. \
                                 This affects build scripts and automation."
                        .to_string(),
                    category: RecommendationCategory::Software,
                    priority: RecommendationPriority::Low,
                    expected_improvement: "20-40% faster script execution".to_string(),
                    how_to_apply: vec![
                        "Check profile scripts: $PROFILE (remove unnecessary modules)".to_string(),
                        "Use PowerShell 7+ for better performance: winget install Microsoft.PowerShell".to_string(),
                        "Disable module auto-loading if not needed".to_string(),
                    ],
                    affected_tests: vec![
                        "PowerShell Scripts".to_string(),
                    ],
                });
            }
        }

        // Check process spawn time
        let process_spawn = run.results.responsiveness.iter()
            .find(|r| r.test_id == "process_spawn");

        if let Some(spawn_result) = process_spawn {
            if spawn_result.value > 50.0 {
                // More than 50ms per process spawn is slow
                recommendations.push(Recommendation {
                    id: "optimize_startup".to_string(),
                    title: "Review Startup Programs".to_string(),
                    description: "Process creation time is elevated. Background services \
                                 and startup programs may be affecting system responsiveness."
                        .to_string(),
                    category: RecommendationCategory::Software,
                    priority: RecommendationPriority::Low,
                    expected_improvement: "10-20% faster process creation".to_string(),
                    how_to_apply: vec![
                        "Open Task Manager > Startup tab".to_string(),
                        "Disable unnecessary startup programs".to_string(),
                        "Review services: services.msc - disable unused services".to_string(),
                        "Check for bloatware and remove if present".to_string(),
                    ],
                    affected_tests: vec![
                        "Process Spawn".to_string(),
                        "Application Launch".to_string(),
                    ],
                });
            }
        }
    }

    fn add_general_recommendations(
        _run: &BenchmarkRun,
        recommendations: &mut Vec<Recommendation>,
    ) {
        // Power plan recommendation (always include if not already added via other checks)
        let has_power_rec = recommendations.iter().any(|r| r.id == "power_plan");
        if !has_power_rec {
            recommendations.push(Recommendation {
                id: "power_plan".to_string(),
                title: "Use High Performance Power Plan".to_string(),
                description: "Windows power plans can limit CPU frequency. \
                             High Performance mode ensures maximum CPU speed during benchmarks and builds."
                    .to_string(),
                category: RecommendationCategory::Software,
                priority: RecommendationPriority::Low,
                expected_improvement: "5-15% faster CPU-intensive tasks".to_string(),
                how_to_apply: vec![
                    "Open Settings > System > Power & battery".to_string(),
                    "Set Power mode to 'Best performance'".to_string(),
                    "Or use Control Panel > Power Options for more options".to_string(),
                    "On laptops, consider 'Balanced' when on battery".to_string(),
                ],
                affected_tests: vec![
                    "Single-Thread CPU".to_string(),
                    "Multi-Thread CPU".to_string(),
                ],
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{BenchmarkRun, CategoryResults, SystemInfo, CpuInfo, MemoryInfo, OsInfo};

    fn create_test_run() -> BenchmarkRun {
        BenchmarkRun::new(
            "TEST-PC".to_string(),
            SystemInfo {
                hostname: "TEST-PC".to_string(),
                cpu: CpuInfo {
                    name: "Test CPU".to_string(),
                    vendor: "Test".to_string(),
                    cores: 8,
                    threads: 16,
                    base_frequency_mhz: 3000,
                    max_frequency_mhz: None,
                    cache_l3_kb: None,
                },
                memory: MemoryInfo {
                    total_bytes: 16 * 1024 * 1024 * 1024,
                    speed_mhz: None,
                    memory_type: None,
                },
                storage: vec![],
                gpu: None,
                os: OsInfo {
                    name: "Windows".to_string(),
                    version: "11".to_string(),
                    build: None,
                },
            },
        )
    }

    #[test]
    fn test_analyze_empty_run() {
        let run = create_test_run();
        let report = RecommendationEngine::analyze(&run, None);

        // Should at least have the power plan recommendation
        assert!(!report.recommendations.is_empty());
    }

    #[test]
    fn test_device_type_detection() {
        let mut run = create_test_run();

        // Test laptop detection
        run.system_info.cpu.name = "Intel Core i7-1265U".to_string();
        let report = RecommendationEngine::analyze(&run, None);
        assert_eq!(report.device_type, DeviceType::Laptop);

        // Test VDI detection
        run.machine_name = "VDI-DESKTOP-001".to_string();
        let report = RecommendationEngine::analyze(&run, None);
        assert_eq!(report.device_type, DeviceType::VDI);
    }

    #[test]
    fn test_priority_sorting() {
        let run = create_test_run();
        let report = RecommendationEngine::analyze(&run, None);

        // Verify recommendations are sorted by priority
        for i in 1..report.recommendations.len() {
            assert!(
                report.recommendations[i - 1].priority <= report.recommendations[i].priority,
                "Recommendations should be sorted by priority"
            );
        }
    }
}
