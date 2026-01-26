use std::process::Command;

use anyhow::Result;

use crate::benchmarks::{Benchmark, Category, ProgressCallback};
use crate::core::Timer;
use crate::models::{TestDetails, TestResult};

/// Process spawn time benchmark
/// Measures time to spawn a simple process 100 times
pub struct ProcessSpawnBenchmark;

impl ProcessSpawnBenchmark {
    pub fn new() -> Self {
        Self
    }

    #[cfg(windows)]
    fn spawn_command() -> Command {
        let mut cmd = Command::new("cmd.exe");
        cmd.args(["/C", "echo test"]);
        cmd
    }

    #[cfg(not(windows))]
    fn spawn_command() -> Command {
        let mut cmd = Command::new("sh");
        cmd.args(["-c", "echo test"]);
        cmd
    }
}

impl Default for ProcessSpawnBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl Benchmark for ProcessSpawnBenchmark {
    fn id(&self) -> &'static str {
        "process_spawn"
    }

    fn name(&self) -> &'static str {
        "Process Spawn Time"
    }

    fn description(&self) -> &'static str {
        "Spawn shell process 100 times - simulates running build tools"
    }

    fn category(&self) -> Category {
        Category::Responsiveness
    }

    fn estimated_duration_secs(&self) -> u32 {
        30
    }

    fn run(&self, progress: &dyn ProgressCallback) -> Result<TestResult> {
        let num_spawns: usize = 100;
        let mut spawn_times_ms: Vec<f64> = Vec::with_capacity(num_spawns);

        progress.update(0.0, "Warming up...");

        // Warmup
        for _ in 0..5 {
            let mut cmd = Self::spawn_command();
            let _ = cmd.output();
        }

        progress.update(0.1, "Measuring spawn times...");

        for i in 0..num_spawns {
            if progress.is_cancelled() {
                return Err(anyhow::anyhow!("Cancelled"));
            }

            let mut cmd = Self::spawn_command();

            let timer = Timer::new();
            let output = cmd.output();
            let elapsed_ms = timer.elapsed_secs() * 1000.0;

            if output.is_ok() {
                spawn_times_ms.push(elapsed_ms);
            }

            if i % 10 == 0 {
                progress.update(
                    0.1 + (i as f32 / num_spawns as f32) * 0.85,
                    &format!("Spawning process {}/{}...", i + 1, num_spawns),
                );
            }
        }

        if spawn_times_ms.is_empty() {
            return Err(anyhow::anyhow!("Failed to spawn any processes"));
        }

        // Calculate statistics
        spawn_times_ms.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let count = spawn_times_ms.len();
        let min = spawn_times_ms[0];
        let max = spawn_times_ms[count - 1];
        let sum: f64 = spawn_times_ms.iter().sum();
        let mean = sum / count as f64;
        let median = spawn_times_ms[count / 2];

        let p99_idx = ((count as f64) * 0.99) as usize;
        let p99 = spawn_times_ms[p99_idx.min(count - 1)];

        let variance: f64 = spawn_times_ms
            .iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>()
            / count as f64;
        let std_dev = variance.sqrt();

        progress.update(1.0, "Complete");

        Ok(TestResult {
            test_id: self.id().to_string(),
            name: self.name().to_string(),
            description: self.description().to_string(),
            value: mean,
            unit: "ms".to_string(),
            details: TestDetails {
                iterations: count as u32,
                duration_secs: sum / 1000.0,
                min,
                max,
                mean,
                median,
                std_dev,
                percentiles: Some(crate::models::Percentiles {
                    p50: median,
                    p75: spawn_times_ms[((count as f64) * 0.75) as usize],
                    p90: spawn_times_ms[((count as f64) * 0.90) as usize],
                    p95: spawn_times_ms[((count as f64) * 0.95) as usize],
                    p99: p99,
                    p999: spawn_times_ms[((count as f64) * 0.999).min(count as f64 - 1.0) as usize],
                }),
            },
        })
    }
}
