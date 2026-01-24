use std::process::Command;

use anyhow::Result;

use crate::benchmarks::{Benchmark, Category, ProgressCallback};
use crate::core::Timer;
use crate::models::{TestDetails, TestResult};

/// Windows Registry operations benchmark
/// Tests registry read/query performance using reg.exe
pub struct RegistryBenchmark;

impl RegistryBenchmark {
    pub fn new() -> Self {
        Self
    }

    fn is_available() -> bool {
        Command::new("reg")
            .arg("/?")
            .output()
            .map(|o| o.status.success() || o.status.code() == Some(1))
            .unwrap_or(false)
    }
}

impl Default for RegistryBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl Benchmark for RegistryBenchmark {
    fn id(&self) -> &'static str {
        "registry"
    }

    fn name(&self) -> &'static str {
        "Registry Operations"
    }

    fn description(&self) -> &'static str {
        "Query Windows registry keys and values"
    }

    fn category(&self) -> Category {
        Category::Responsiveness
    }

    fn estimated_duration_secs(&self) -> u32 {
        30
    }

    fn run(&self, progress: &dyn ProgressCallback) -> Result<TestResult> {
        if !Self::is_available() {
            return Err(anyhow::anyhow!("reg.exe not available (Windows only)"));
        }

        progress.update(0.05, "Preparing registry benchmark...");

        // Common registry paths to query (read-only, safe operations)
        let registry_queries = [
            // System information
            (r"HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion", "ProductName"),
            (r"HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion", "CurrentBuild"),
            (r"HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion", "EditionID"),
            // Hardware info
            (r"HKLM\HARDWARE\DESCRIPTION\System\CentralProcessor\0", "ProcessorNameString"),
            (r"HKLM\HARDWARE\DESCRIPTION\System\BIOS", "SystemManufacturer"),
            // Environment
            (r"HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Environment", "Path"),
            (r"HKCU\Environment", "Path"),
            // Software keys
            (r"HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion", "ProgramFilesDir"),
            (r"HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Shell Folders", "Desktop"),
        ];

        let mut query_times: Vec<f64> = Vec::new();
        let mut enum_times: Vec<f64> = Vec::new();

        // Test 1: Individual value queries
        progress.update(0.1, "Testing registry value queries...");

        for round in 0..5 {
            if progress.is_cancelled() {
                return Err(anyhow::anyhow!("Cancelled"));
            }

            progress.update(
                0.1 + (round as f32 / 5.0) * 0.4,
                &format!("Query round {}/5...", round + 1),
            );

            for (key, value) in &registry_queries {
                let timer = Timer::new();
                let _ = Command::new("reg")
                    .args(["query", key, "/v", value])
                    .output();
                query_times.push(timer.elapsed_secs() * 1000.0);
            }
        }

        // Test 2: Key enumeration
        progress.update(0.5, "Testing registry key enumeration...");

        let enum_keys = [
            r"HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run",
            r"HKLM\SYSTEM\CurrentControlSet\Services",
        ];

        for round in 0..5 {
            if progress.is_cancelled() {
                return Err(anyhow::anyhow!("Cancelled"));
            }

            progress.update(
                0.5 + (round as f32 / 5.0) * 0.4,
                &format!("Enumeration round {}/5...", round + 1),
            );

            for key in &enum_keys {
                let timer = Timer::new();
                let _ = Command::new("reg")
                    .args(["query", key])
                    .output();
                enum_times.push(timer.elapsed_secs() * 1000.0);
            }
        }

        // Calculate statistics
        let avg_query = query_times.iter().sum::<f64>() / query_times.len() as f64;
        let avg_enum = enum_times.iter().sum::<f64>() / enum_times.len() as f64;
        let avg_combined = (avg_query + avg_enum) / 2.0;

        // Score: faster is better
        // <10ms = 300, <20ms = 250, <50ms = 200, <100ms = 150, >100ms = 100
        let score = if avg_combined < 10.0 {
            300
        } else if avg_combined < 20.0 {
            250
        } else if avg_combined < 50.0 {
            200
        } else if avg_combined < 100.0 {
            150
        } else {
            100
        };

        let all_times: Vec<f64> = [query_times, enum_times].concat();
        let min = all_times.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = all_times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        progress.update(1.0, "Complete");

        Ok(TestResult {
            test_id: self.id().to_string(),
            name: self.name().to_string(),
            description: format!(
                "{} (query: {:.1}ms, enum: {:.1}ms)",
                self.description(),
                avg_query,
                avg_enum
            ),
            value: avg_combined,
            unit: "ms".to_string(),
            score,
            max_score: 300,
            details: TestDetails {
                iterations: all_times.len() as u32,
                duration_secs: all_times.iter().sum::<f64>() / 1000.0,
                min,
                max,
                mean: avg_combined,
                median: avg_combined,
                std_dev: 0.0,
                percentiles: None,
            },
        })
    }
}
