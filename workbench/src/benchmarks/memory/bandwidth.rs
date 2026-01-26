use anyhow::Result;
use rayon::prelude::*;

use crate::benchmarks::{Benchmark, BenchmarkConfig, Category, ProgressCallback};
use crate::core::Timer;
use crate::models::{TestDetails, TestResult};

/// Memory bandwidth benchmark using multi-threaded memory copy
/// Measures GB/s throughput
pub struct MemoryBandwidthBenchmark;

impl MemoryBandwidthBenchmark {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MemoryBandwidthBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl Benchmark for MemoryBandwidthBenchmark {
    fn id(&self) -> &'static str {
        "memory_bandwidth"
    }

    fn name(&self) -> &'static str {
        "Memory Bandwidth"
    }

    fn description(&self) -> &'static str {
        "Multi-threaded memory copy - measures bandwidth in GB/s"
    }

    fn category(&self) -> Category {
        Category::Responsiveness
    }

    fn estimated_duration_secs(&self) -> u32 {
        15
    }

    fn is_synthetic(&self) -> bool {
        true
    }

    fn run(&self, progress: &dyn ProgressCallback, config: &BenchmarkConfig) -> Result<TestResult> {
        let num_threads = rayon::current_num_threads();
        let per_thread_size: usize = config.mem_bandwidth_buffer_mb as usize * 1024 * 1024;
        let total_size = per_thread_size * num_threads;

        progress.update(0.0, &format!("Allocating {} GB across {} threads...",
            total_size / (1024 * 1024 * 1024), num_threads));

        // Allocate source and destination buffers for each thread
        let mut sources: Vec<Vec<u8>> = (0..num_threads)
            .map(|i| {
                let mut v = vec![0u8; per_thread_size];
                // Initialize with different values to prevent optimization
                for (j, byte) in v.iter_mut().enumerate() {
                    *byte = ((i + j) % 256) as u8;
                }
                v
            })
            .collect();

        let mut destinations: Vec<Vec<u8>> = (0..num_threads)
            .map(|_| vec![0u8; per_thread_size])
            .collect();

        progress.update(0.1, "Warming up...");

        // Warmup
        sources
            .par_iter()
            .zip(destinations.par_iter_mut())
            .for_each(|(src, dst)| {
                dst.copy_from_slice(src);
            });

        progress.update(0.2, "Measuring memory bandwidth...");

        let mut bandwidths: Vec<f64> = Vec::new();
        let num_runs = config.iterations as usize;
        let copies_per_run = 3; // Multiple copies per timing run

        for run in 0..num_runs {
            if progress.is_cancelled() {
                return Err(anyhow::anyhow!("Cancelled"));
            }

            let timer = Timer::new();

            for _ in 0..copies_per_run {
                // Parallel copy: all threads copy simultaneously
                sources
                    .par_iter()
                    .zip(destinations.par_iter_mut())
                    .for_each(|(src, dst)| {
                        dst.copy_from_slice(src);
                    });
            }

            let elapsed = timer.elapsed_secs();
            let bytes_copied = total_size * copies_per_run * 2; // Read + Write
            let gb_per_sec = (bytes_copied as f64 / (1024.0 * 1024.0 * 1024.0)) / elapsed;
            bandwidths.push(gb_per_sec);

            progress.update(
                0.2 + (run as f32 / num_runs as f32) * 0.75,
                &format!("Run {}/{}: {:.1} GB/s", run + 1, num_runs, gb_per_sec),
            );
        }

        // Calculate statistics
        bandwidths.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let min = bandwidths[0];
        let max = bandwidths[bandwidths.len() - 1];
        let sum: f64 = bandwidths.iter().sum();
        let mean = sum / bandwidths.len() as f64;
        let median = bandwidths[bandwidths.len() / 2];

        let variance: f64 = bandwidths
            .iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>()
            / bandwidths.len() as f64;
        let std_dev = variance.sqrt();

        progress.update(1.0, "Complete");

        Ok(TestResult {
            test_id: self.id().to_string(),
            name: self.name().to_string(),
            description: self.description().to_string(),
            value: median,
            unit: "GB/s".to_string(),
            details: TestDetails {
                iterations: (num_runs * copies_per_run) as u32,
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
