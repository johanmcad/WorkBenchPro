use anyhow::Result;

use crate::benchmarks::{Benchmark, Category, ProgressCallback};
use crate::core::{system_command, Timer};
use crate::models::{TestDetails, TestResult};

/// Windows Task Scheduler benchmark
/// Tests task scheduler query performance using schtasks.exe
pub struct TaskSchedulerBenchmark;

impl TaskSchedulerBenchmark {
    pub fn new() -> Self {
        Self
    }

    fn is_available() -> bool {
        system_command("schtasks.exe")
            .arg("/?")
            .output()
            .map(|o| o.status.success() || o.status.code() == Some(0))
            .unwrap_or(false)
    }
}

impl Default for TaskSchedulerBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl Benchmark for TaskSchedulerBenchmark {
    fn id(&self) -> &'static str {
        "taskscheduler"
    }

    fn name(&self) -> &'static str {
        "Task Scheduler"
    }

    fn description(&self) -> &'static str {
        "Query Windows Task Scheduler using schtasks"
    }

    fn category(&self) -> Category {
        Category::Responsiveness
    }

    fn estimated_duration_secs(&self) -> u32 {
        30
    }

    fn run(&self, progress: &dyn ProgressCallback) -> Result<TestResult> {
        if !Self::is_available() {
            return Err(anyhow::anyhow!("schtasks not available (Windows only)"));
        }

        progress.update(0.05, "Preparing task scheduler benchmark...");

        let mut list_times: Vec<f64> = Vec::new();
        let mut query_times: Vec<f64> = Vec::new();
        let mut verbose_times: Vec<f64> = Vec::new();

        // Test 1: List all tasks (basic)
        progress.update(0.1, "Listing scheduled tasks...");

        for round in 0..5 {
            if progress.is_cancelled() {
                return Err(anyhow::anyhow!("Cancelled"));
            }

            let timer = Timer::new();
            let _ = system_command("schtasks.exe")
                .args(["/Query", "/FO", "LIST"])
                .output();
            list_times.push(timer.elapsed_secs() * 1000.0);

            progress.update(
                0.1 + (round as f32 / 5.0) * 0.3,
                &format!("List tasks round {}/5...", round + 1),
            );
        }

        // Test 2: Query specific task folders
        progress.update(0.4, "Querying task folders...");

        let task_folders = [
            "\\Microsoft\\Windows\\Maintenance",
            "\\Microsoft\\Windows\\WindowsUpdate",
            "\\Microsoft\\Windows\\Defrag",
        ];

        for round in 0..3 {
            if progress.is_cancelled() {
                return Err(anyhow::anyhow!("Cancelled"));
            }

            for folder in &task_folders {
                let timer = Timer::new();
                let _ = system_command("schtasks.exe")
                    .args(["/Query", "/TN", folder, "/FO", "LIST"])
                    .output();
                query_times.push(timer.elapsed_secs() * 1000.0);
            }

            progress.update(
                0.4 + (round as f32 / 3.0) * 0.3,
                &format!("Query folders round {}/3...", round + 1),
            );
        }

        // Test 3: Verbose query (includes more details)
        progress.update(0.7, "Verbose task queries...");

        for round in 0..3 {
            if progress.is_cancelled() {
                return Err(anyhow::anyhow!("Cancelled"));
            }

            let timer = Timer::new();
            let _ = system_command("schtasks.exe")
                .args(["/Query", "/FO", "LIST", "/V"])
                .output();
            verbose_times.push(timer.elapsed_secs() * 1000.0);

            progress.update(
                0.7 + (round as f32 / 3.0) * 0.25,
                &format!("Verbose query round {}/3...", round + 1),
            );
        }

        // Calculate statistics
        let avg_list = list_times.iter().sum::<f64>() / list_times.len() as f64;
        let avg_query = query_times.iter().sum::<f64>() / query_times.len() as f64;
        let avg_verbose = verbose_times.iter().sum::<f64>() / verbose_times.len() as f64;
        let avg_combined = (avg_list + avg_query + avg_verbose) / 3.0;

        let all_times: Vec<f64> = [list_times, query_times, verbose_times].concat();
        let min = all_times.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = all_times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        progress.update(1.0, "Complete");

        Ok(TestResult {
            test_id: self.id().to_string(),
            name: self.name().to_string(),
            description: format!(
                "{} (list: {:.0}ms, query: {:.0}ms, verbose: {:.0}ms)",
                self.description(),
                avg_list,
                avg_query,
                avg_verbose
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
