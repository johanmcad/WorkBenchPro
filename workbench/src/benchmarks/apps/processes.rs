use std::process::Command;

use anyhow::Result;

use crate::benchmarks::{Benchmark, BenchmarkConfig, Category, ProgressCallback};
use crate::core::{system_command, Timer};
use crate::models::{TestDetails, TestResult};

/// Process Management benchmark
/// Tests process listing and querying using tasklist/ps
pub struct ProcessesBenchmark;

impl ProcessesBenchmark {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ProcessesBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl Benchmark for ProcessesBenchmark {
    fn id(&self) -> &'static str {
        "processes"
    }

    fn name(&self) -> &'static str {
        "Process Management"
    }

    fn description(&self) -> &'static str {
        "List and query running processes"
    }

    fn category(&self) -> Category {
        Category::Responsiveness
    }

    fn estimated_duration_secs(&self) -> u32 {
        30
    }

    fn run(&self, progress: &dyn ProgressCallback, _config: &BenchmarkConfig) -> Result<TestResult> {
        progress.update(0.05, "Preparing process benchmark...");

        let is_windows = cfg!(target_os = "windows");

        let mut list_times: Vec<f64> = Vec::new();
        let mut verbose_times: Vec<f64> = Vec::new();
        let mut filter_times: Vec<f64> = Vec::new();

        // Test 1: Basic process list
        progress.update(0.1, "Listing processes...");

        for round in 0..5 {
            if progress.is_cancelled() {
                return Err(anyhow::anyhow!("Cancelled"));
            }

            let timer = Timer::new();
            if is_windows {
                let _ = system_command("tasklist.exe")
                    .output();
            } else {
                let _ = Command::new("ps")
                    .args(["aux"])
                    .output();
            }
            list_times.push(timer.elapsed_secs() * 1000.0);

            progress.update(
                0.1 + (round as f32 / 5.0) * 0.3,
                &format!("Process list round {}/5...", round + 1),
            );
        }

        // Test 2: Verbose/detailed process list
        progress.update(0.4, "Detailed process query...");

        for round in 0..5 {
            if progress.is_cancelled() {
                return Err(anyhow::anyhow!("Cancelled"));
            }

            let timer = Timer::new();
            if is_windows {
                let _ = system_command("tasklist.exe")
                    .args(["/V", "/FO", "LIST"])
                    .output();
            } else {
                let _ = Command::new("ps")
                    .args(["auxf"])
                    .output();
            }
            verbose_times.push(timer.elapsed_secs() * 1000.0);

            progress.update(
                0.4 + (round as f32 / 5.0) * 0.3,
                &format!("Verbose list round {}/5...", round + 1),
            );
        }

        // Test 3: Filtered process queries
        progress.update(0.7, "Filtered process queries...");

        let filters = if is_windows {
            vec![
                vec!["tasklist", "/FI", "STATUS eq RUNNING"],
                vec!["tasklist", "/FI", "MEMUSAGE gt 10000"],
                vec!["tasklist", "/M"],  // List with modules
            ]
        } else {
            vec![
                vec!["ps", "-eo", "pid,comm,%mem,%cpu"],
                vec!["ps", "--sort=-%mem", "-eo", "pid,comm,%mem"],
                vec!["ps", "-ef"],
            ]
        };

        for round in 0..3 {
            if progress.is_cancelled() {
                return Err(anyhow::anyhow!("Cancelled"));
            }

            for filter_args in &filters {
                let timer = Timer::new();
                let _ = Command::new(filter_args[0])
                    .args(&filter_args[1..])
                    .output();
                filter_times.push(timer.elapsed_secs() * 1000.0);
            }

            progress.update(
                0.7 + (round as f32 / 3.0) * 0.25,
                &format!("Filter round {}/3...", round + 1),
            );
        }

        // Calculate statistics
        let avg_list = list_times.iter().sum::<f64>() / list_times.len() as f64;
        let avg_verbose = verbose_times.iter().sum::<f64>() / verbose_times.len() as f64;
        let avg_filter = filter_times.iter().sum::<f64>() / filter_times.len() as f64;
        let avg_combined = (avg_list + avg_verbose + avg_filter) / 3.0;

        let all_times: Vec<f64> = [list_times, verbose_times, filter_times].concat();
        let min = all_times.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = all_times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        progress.update(1.0, "Complete");

        Ok(TestResult {
            test_id: self.id().to_string(),
            name: self.name().to_string(),
            description: format!(
                "{} (list: {:.0}ms, verbose: {:.0}ms, filter: {:.0}ms)",
                self.description(),
                avg_list,
                avg_verbose,
                avg_filter
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
