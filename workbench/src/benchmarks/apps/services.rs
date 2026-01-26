use anyhow::Result;

use crate::benchmarks::{Benchmark, BenchmarkConfig, Category, ProgressCallback};
use crate::core::{system_command, Timer};
use crate::models::{TestDetails, TestResult};

/// Windows Services benchmark
/// Tests service query performance using sc.exe
pub struct ServicesBenchmark;

impl ServicesBenchmark {
    pub fn new() -> Self {
        Self
    }

    fn is_available() -> bool {
        system_command("sc.exe")
            .arg("query")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
}

impl Default for ServicesBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl Benchmark for ServicesBenchmark {
    fn id(&self) -> &'static str {
        "services"
    }

    fn name(&self) -> &'static str {
        "Windows Services"
    }

    fn description(&self) -> &'static str {
        "Query Windows services using sc.exe"
    }

    fn category(&self) -> Category {
        Category::Responsiveness
    }

    fn estimated_duration_secs(&self) -> u32 {
        30
    }

    fn run(&self, progress: &dyn ProgressCallback, _config: &BenchmarkConfig) -> Result<TestResult> {
        if !Self::is_available() {
            return Err(anyhow::anyhow!("sc.exe not available (Windows only)"));
        }

        progress.update(0.05, "Preparing services benchmark...");

        let mut query_all_times: Vec<f64> = Vec::new();
        let mut query_specific_times: Vec<f64> = Vec::new();
        let mut query_config_times: Vec<f64> = Vec::new();

        // Common Windows services to query
        let services = [
            "wuauserv",     // Windows Update
            "WinDefend",    // Windows Defender
            "Spooler",      // Print Spooler
            "BITS",         // Background Intelligent Transfer
            "W32Time",      // Windows Time
            "Dhcp",         // DHCP Client
            "Dnscache",     // DNS Client
            "EventLog",     // Windows Event Log
        ];

        // Test 1: Query all services
        progress.update(0.1, "Querying all services...");

        for round in 0..5 {
            if progress.is_cancelled() {
                return Err(anyhow::anyhow!("Cancelled"));
            }

            let timer = Timer::new();
            let _ = system_command("sc.exe")
                .args(["query", "type=", "service", "state=", "all"])
                .output();
            query_all_times.push(timer.elapsed_secs() * 1000.0);

            progress.update(
                0.1 + (round as f32 / 5.0) * 0.3,
                &format!("Query all round {}/5...", round + 1),
            );
        }

        // Test 2: Query specific services
        progress.update(0.4, "Querying specific services...");

        for round in 0..3 {
            if progress.is_cancelled() {
                return Err(anyhow::anyhow!("Cancelled"));
            }

            for service in &services {
                let timer = Timer::new();
                let _ = system_command("sc.exe")
                    .args(["query", service])
                    .output();
                query_specific_times.push(timer.elapsed_secs() * 1000.0);
            }

            progress.update(
                0.4 + (round as f32 / 3.0) * 0.3,
                &format!("Query specific round {}/3...", round + 1),
            );
        }

        // Test 3: Query service configurations
        progress.update(0.7, "Querying service configurations...");

        for round in 0..3 {
            if progress.is_cancelled() {
                return Err(anyhow::anyhow!("Cancelled"));
            }

            for service in &services[..4] {
                let timer = Timer::new();
                let _ = system_command("sc.exe")
                    .args(["qc", service])
                    .output();
                query_config_times.push(timer.elapsed_secs() * 1000.0);
            }

            progress.update(
                0.7 + (round as f32 / 3.0) * 0.25,
                &format!("Query config round {}/3...", round + 1),
            );
        }

        // Calculate statistics
        let avg_all = query_all_times.iter().sum::<f64>() / query_all_times.len() as f64;
        let avg_specific = query_specific_times.iter().sum::<f64>() / query_specific_times.len() as f64;
        let avg_config = query_config_times.iter().sum::<f64>() / query_config_times.len() as f64;
        let avg_combined = (avg_all + avg_specific + avg_config) / 3.0;

        let all_times: Vec<f64> = [query_all_times, query_specific_times, query_config_times].concat();
        let min = all_times.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = all_times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        progress.update(1.0, "Complete");

        Ok(TestResult {
            test_id: self.id().to_string(),
            name: self.name().to_string(),
            description: format!(
                "{} (all: {:.0}ms, specific: {:.1}ms, config: {:.1}ms)",
                self.description(),
                avg_all,
                avg_specific,
                avg_config
            ),
            value: avg_combined,
            unit: "ms".to_string(),
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
