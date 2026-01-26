use anyhow::Result;

use crate::benchmarks::{Benchmark, Category, ProgressCallback};
use crate::core::{system_command, Timer};
use crate::models::{TestDetails, TestResult};

/// WMIC/System Information benchmark
/// Tests WMI queries for system information
pub struct WmicBenchmark;

impl WmicBenchmark {
    pub fn new() -> Self {
        Self
    }

    fn is_available() -> bool {
        // Try wmic first, fall back to PowerShell Get-WmiObject
        system_command("wmic.exe")
            .arg("os")
            .arg("get")
            .arg("caption")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
}

impl Default for WmicBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl Benchmark for WmicBenchmark {
    fn id(&self) -> &'static str {
        "wmic"
    }

    fn name(&self) -> &'static str {
        "System Info (WMIC)"
    }

    fn description(&self) -> &'static str {
        "Query system information using WMIC/WMI"
    }

    fn category(&self) -> Category {
        Category::Responsiveness
    }

    fn estimated_duration_secs(&self) -> u32 {
        45
    }

    fn run(&self, progress: &dyn ProgressCallback) -> Result<TestResult> {
        if !Self::is_available() {
            return Err(anyhow::anyhow!("WMIC not available (Windows only)"));
        }

        progress.update(0.05, "Preparing WMIC benchmark...");

        // WMIC queries to run
        let wmic_queries = [
            ("os", "caption,version,buildnumber"),
            ("cpu", "name,numberofcores,maxclockspeed"),
            ("memorychip", "capacity,speed"),
            ("diskdrive", "model,size"),
            ("bios", "manufacturer,version"),
            ("baseboard", "manufacturer,product"),
            ("nic", "name,macaddress"),
        ];

        let mut query_times: Vec<f64> = Vec::new();
        let mut list_times: Vec<f64> = Vec::new();

        // Test 1: Individual WMI queries
        progress.update(0.1, "Running WMI queries...");

        for round in 0..3 {
            if progress.is_cancelled() {
                return Err(anyhow::anyhow!("Cancelled"));
            }

            for (class, fields) in &wmic_queries {
                let timer = Timer::new();
                let _ = system_command("wmic.exe")
                    .args([class, "get", fields, "/format:list"])
                    .output();
                query_times.push(timer.elapsed_secs() * 1000.0);
            }

            progress.update(
                0.1 + (round as f32 / 3.0) * 0.5,
                &format!("WMI query round {}/3...", round + 1),
            );
        }

        // Test 2: Process list via WMIC
        progress.update(0.6, "Querying process list...");

        for round in 0..5 {
            if progress.is_cancelled() {
                return Err(anyhow::anyhow!("Cancelled"));
            }

            let timer = Timer::new();
            let _ = system_command("wmic.exe")
                .args(["process", "get", "name,processid,workingsetsize", "/format:list"])
                .output();
            list_times.push(timer.elapsed_secs() * 1000.0);

            progress.update(
                0.6 + (round as f32 / 5.0) * 0.35,
                &format!("Process list round {}/5...", round + 1),
            );
        }

        // Calculate statistics
        let avg_query = query_times.iter().sum::<f64>() / query_times.len() as f64;
        let avg_list = list_times.iter().sum::<f64>() / list_times.len() as f64;
        let avg_combined = (avg_query + avg_list) / 2.0;

        let all_times: Vec<f64> = [query_times, list_times].concat();
        let min = all_times.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = all_times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        progress.update(1.0, "Complete");

        Ok(TestResult {
            test_id: self.id().to_string(),
            name: self.name().to_string(),
            description: format!(
                "{} (query: {:.0}ms, list: {:.0}ms)",
                self.description(),
                avg_query,
                avg_list
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
