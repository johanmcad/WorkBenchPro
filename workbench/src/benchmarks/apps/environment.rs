use std::env;
use std::process::Command;

use anyhow::Result;

use crate::benchmarks::{Benchmark, BenchmarkConfig, Category, ProgressCallback};
use crate::core::{system_command, Timer};
use crate::models::{TestDetails, TestResult};

/// Environment Variables benchmark
/// Tests reading and querying environment variables
pub struct EnvironmentBenchmark;

impl EnvironmentBenchmark {
    pub fn new() -> Self {
        Self
    }
}

impl Default for EnvironmentBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl Benchmark for EnvironmentBenchmark {
    fn id(&self) -> &'static str {
        "environment"
    }

    fn name(&self) -> &'static str {
        "Environment Variables"
    }

    fn description(&self) -> &'static str {
        "Read and query environment variables"
    }

    fn category(&self) -> Category {
        Category::Responsiveness
    }

    fn estimated_duration_secs(&self) -> u32 {
        20
    }

    fn run(&self, progress: &dyn ProgressCallback, _config: &BenchmarkConfig) -> Result<TestResult> {
        progress.update(0.05, "Preparing environment benchmark...");

        let is_windows = cfg!(target_os = "windows");

        // Common environment variables to query
        let env_vars = [
            "PATH",
            "HOME",
            "USER",
            "TEMP",
            "TMP",
            "USERPROFILE",
            "APPDATA",
            "LOCALAPPDATA",
            "PROGRAMFILES",
            "WINDIR",
            "SYSTEMROOT",
            "COMPUTERNAME",
            "USERNAME",
            "OS",
            "PROCESSOR_ARCHITECTURE",
        ];

        let mut native_times: Vec<f64> = Vec::new();
        let mut command_times: Vec<f64> = Vec::new();
        let mut all_vars_times: Vec<f64> = Vec::new();

        // Test 1: Native Rust env::var queries
        progress.update(0.1, "Native environment queries...");

        for round in 0..10 {
            if progress.is_cancelled() {
                return Err(anyhow::anyhow!("Cancelled"));
            }

            let timer = Timer::new();
            for var in &env_vars {
                let _ = env::var(var);
            }
            native_times.push(timer.elapsed_secs() * 1000.0);

            progress.update(
                0.1 + (round as f32 / 10.0) * 0.3,
                &format!("Native query round {}/10...", round + 1),
            );
        }

        // Test 2: Command-based environment queries
        progress.update(0.4, "Command-based queries...");

        for round in 0..5 {
            if progress.is_cancelled() {
                return Err(anyhow::anyhow!("Cancelled"));
            }

            let timer = Timer::new();
            if is_windows {
                // Use SET command to display environment
                let _ = system_command("cmd.exe")
                    .args(["/c", "set"])
                    .output();
            } else {
                let _ = Command::new("printenv")
                    .output();
            }
            command_times.push(timer.elapsed_secs() * 1000.0);

            progress.update(
                0.4 + (round as f32 / 5.0) * 0.3,
                &format!("Command query round {}/5...", round + 1),
            );
        }

        // Test 3: Iterate all environment variables
        progress.update(0.7, "Iterating all variables...");

        for round in 0..10 {
            if progress.is_cancelled() {
                return Err(anyhow::anyhow!("Cancelled"));
            }

            let timer = Timer::new();
            let mut count = 0;
            for (key, value) in env::vars() {
                // Process each variable
                let _ = key.len() + value.len();
                count += 1;
            }
            let _ = count; // Prevent optimization
            all_vars_times.push(timer.elapsed_secs() * 1000.0);

            progress.update(
                0.7 + (round as f32 / 10.0) * 0.25,
                &format!("Iterate round {}/10...", round + 1),
            );
        }

        // Calculate statistics
        let avg_native = native_times.iter().sum::<f64>() / native_times.len() as f64;
        let avg_command = command_times.iter().sum::<f64>() / command_times.len() as f64;
        let avg_all = all_vars_times.iter().sum::<f64>() / all_vars_times.len() as f64;
        let avg_combined = (avg_native + avg_command + avg_all) / 3.0;

        let all_times: Vec<f64> = [native_times, command_times, all_vars_times].concat();
        let min = all_times.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = all_times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        progress.update(1.0, "Complete");

        Ok(TestResult {
            test_id: self.id().to_string(),
            name: self.name().to_string(),
            description: format!(
                "{} (native: {:.2}ms, cmd: {:.0}ms, iter: {:.2}ms)",
                self.description(),
                avg_native,
                avg_command,
                avg_all
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
