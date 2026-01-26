use anyhow::Result;
use lz4_flex::{compress_prepend_size, decompress_size_prepended};
use rand::Rng;

use crate::benchmarks::{Benchmark, BenchmarkConfig, Category, ProgressCallback};
use crate::core::Timer;
use crate::models::{TestDetails, TestResult};

/// Single-thread compute benchmark using LZ4 compression
/// Simulates single-file compilation
pub struct SingleThreadBenchmark;

impl SingleThreadBenchmark {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SingleThreadBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl Benchmark for SingleThreadBenchmark {
    fn id(&self) -> &'static str {
        "single_thread_compute"
    }

    fn name(&self) -> &'static str {
        "Single-Thread Compute"
    }

    fn description(&self) -> &'static str {
        "LZ4 compress/decompress loop - simulates single-file compilation"
    }

    fn category(&self) -> Category {
        Category::BuildPerformance
    }

    fn estimated_duration_secs(&self) -> u32 {
        30
    }

    fn is_synthetic(&self) -> bool {
        true
    }

    fn run(&self, progress: &dyn ProgressCallback, config: &BenchmarkConfig) -> Result<TestResult> {
        let chunk_size: usize = 64 * 1024; // 64KB chunks
        let total_data: usize = config.cpu_single_thread_mb as usize * 1024 * 1024;
        let iterations = total_data / chunk_size;

        progress.update(0.0, "Generating test data...");

        // Generate random data (compressible mix)
        let mut rng = rand::thread_rng();
        let mut data = vec![0u8; chunk_size];
        for i in 0..chunk_size {
            // Mix of random and repeating patterns for realistic compression
            data[i] = if i % 4 == 0 {
                rng.gen()
            } else {
                data[i.saturating_sub(1)]
            };
        }

        progress.update(0.1, "Warming up...");

        // Warmup
        for _ in 0..10 {
            let compressed = compress_prepend_size(&data);
            let _ = decompress_size_prepended(&compressed);
        }

        progress.update(0.2, "Running benchmark...");

        let mut throughputs: Vec<f64> = Vec::new();
        let num_runs = config.iterations as usize;

        for run in 0..num_runs {
            if progress.is_cancelled() {
                return Err(anyhow::anyhow!("Cancelled"));
            }

            let timer = Timer::new();
            let mut bytes_processed: usize = 0;

            for i in 0..iterations {
                let compressed = compress_prepend_size(&data);
                let decompressed = decompress_size_prepended(&compressed)?;
                bytes_processed += data.len() + decompressed.len();

                if i % 500 == 0 {
                    let run_progress = 0.2 + (run as f32 / num_runs as f32) * 0.75
                        + (i as f32 / iterations as f32) * (0.75 / num_runs as f32);
                    progress.update(run_progress, &format!("Run {}/{}: Processing...", run + 1, num_runs));
                }
            }

            let elapsed = timer.elapsed_secs();
            let mb_per_sec = (bytes_processed as f64 / (1024.0 * 1024.0)) / elapsed;
            throughputs.push(mb_per_sec);
        }

        // Calculate statistics
        throughputs.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let min = throughputs[0];
        let max = throughputs[throughputs.len() - 1];
        let sum: f64 = throughputs.iter().sum();
        let mean = sum / throughputs.len() as f64;
        let median = throughputs[throughputs.len() / 2];

        let variance: f64 = throughputs
            .iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>()
            / throughputs.len() as f64;
        let std_dev = variance.sqrt();

        progress.update(1.0, "Complete");

        Ok(TestResult {
            test_id: self.id().to_string(),
            name: self.name().to_string(),
            description: self.description().to_string(),
            value: median,
            unit: "MB/s".to_string(),
            details: TestDetails {
                iterations: (iterations * num_runs) as u32,
                duration_secs: sum / mean,
                min,
                max,
                mean,
                median,
                std_dev,
                percentiles: None,
            },
        })
    }
}
