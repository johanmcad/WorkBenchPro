use anyhow::Result;

use crate::benchmarks::{Benchmark, BenchmarkConfig, Category, ProgressCallback};
use crate::core::{system_command, Timer};
use crate::models::{TestDetails, TestResult};

/// Windows Event Log benchmark
/// Tests event log query performance using wevtutil.exe
pub struct EventLogBenchmark;

impl EventLogBenchmark {
    pub fn new() -> Self {
        Self
    }

    fn is_available() -> bool {
        system_command("wevtutil.exe")
            .arg("/?")
            .output()
            .map(|o| o.status.success() || o.status.code() == Some(1))
            .unwrap_or(false)
    }
}

impl Default for EventLogBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl Benchmark for EventLogBenchmark {
    fn id(&self) -> &'static str {
        "eventlog"
    }

    fn name(&self) -> &'static str {
        "Event Log Query"
    }

    fn description(&self) -> &'static str {
        "Query Windows Event Logs using wevtutil"
    }

    fn category(&self) -> Category {
        Category::Responsiveness
    }

    fn estimated_duration_secs(&self) -> u32 {
        45
    }

    fn run(&self, progress: &dyn ProgressCallback, _config: &BenchmarkConfig) -> Result<TestResult> {
        if !Self::is_available() {
            return Err(anyhow::anyhow!("wevtutil not available (Windows only)"));
        }

        progress.update(0.05, "Preparing event log benchmark...");

        // Common event logs to query
        let event_logs = [
            "System",
            "Application",
            "Security",
            "Setup",
        ];

        let mut list_times: Vec<f64> = Vec::new();
        let mut query_times: Vec<f64> = Vec::new();
        let mut info_times: Vec<f64> = Vec::new();

        // Test 1: List available logs
        progress.update(0.1, "Listing event logs...");

        for round in 0..5 {
            if progress.is_cancelled() {
                return Err(anyhow::anyhow!("Cancelled"));
            }

            let timer = Timer::new();
            let _ = system_command("wevtutil.exe")
                .args(["el"])  // enumerate logs
                .output();
            list_times.push(timer.elapsed_secs() * 1000.0);

            progress.update(
                0.1 + (round as f32 / 5.0) * 0.2,
                &format!("List logs round {}/5...", round + 1),
            );
        }

        // Test 2: Get log information
        progress.update(0.3, "Getting log information...");

        for round in 0..3 {
            if progress.is_cancelled() {
                return Err(anyhow::anyhow!("Cancelled"));
            }

            for log in &event_logs {
                let timer = Timer::new();
                let _ = system_command("wevtutil.exe")
                    .args(["gli", log])  // get log info
                    .output();
                info_times.push(timer.elapsed_secs() * 1000.0);
            }

            progress.update(
                0.3 + (round as f32 / 3.0) * 0.3,
                &format!("Log info round {}/3...", round + 1),
            );
        }

        // Test 3: Query recent events (limited to 10 events for speed)
        progress.update(0.6, "Querying recent events...");

        for round in 0..3 {
            if progress.is_cancelled() {
                return Err(anyhow::anyhow!("Cancelled"));
            }

            for log in &event_logs[..2] {  // Only System and Application
                let timer = Timer::new();
                let _ = system_command("wevtutil.exe")
                    .args([
                        "qe",           // query events
                        log,
                        "/c:10",        // count: 10 events
                        "/rd:true",     // read direction: newest first
                        "/f:text",      // format: text
                    ])
                    .output();
                query_times.push(timer.elapsed_secs() * 1000.0);
            }

            progress.update(
                0.6 + (round as f32 / 3.0) * 0.35,
                &format!("Query events round {}/3...", round + 1),
            );
        }

        // Calculate statistics
        let avg_list = list_times.iter().sum::<f64>() / list_times.len() as f64;
        let avg_info = info_times.iter().sum::<f64>() / info_times.len() as f64;
        let avg_query = query_times.iter().sum::<f64>() / query_times.len() as f64;
        let avg_combined = (avg_list + avg_info + avg_query) / 3.0;

        let all_times: Vec<f64> = [list_times, info_times, query_times].concat();
        let min = all_times.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = all_times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        progress.update(1.0, "Complete");

        Ok(TestResult {
            test_id: self.id().to_string(),
            name: self.name().to_string(),
            description: format!(
                "{} (list: {:.0}ms, info: {:.0}ms, query: {:.0}ms)",
                self.description(),
                avg_list,
                avg_info,
                avg_query
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
