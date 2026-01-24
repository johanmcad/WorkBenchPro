use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use anyhow::Result;
use lz4_flex::{compress_prepend_size, decompress_size_prepended};
use rand::Rng;
use rayon::prelude::*;

use crate::benchmarks::{Benchmark, Category, ProgressCallback};
use crate::core::Timer;
use crate::models::{TestDetails, TestResult};
use crate::scoring::thresholds;

/// Multi-thread compute benchmark using LZ4 compression across all cores
/// Simulates parallel build
pub struct MultiThreadBenchmark;

impl MultiThreadBenchmark {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MultiThreadBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl Benchmark for MultiThreadBenchmark {
    fn id(&self) -> &'static str {
        "multi_thread_compute"
    }

    fn name(&self) -> &'static str {
        "Multi-Thread Compute"
    }

    fn description(&self) -> &'static str {
        "Parallel LZ4 compress/decompress - simulates parallel build"
    }

    fn category(&self) -> Category {
        Category::BuildPerformance
    }

    fn estimated_duration_secs(&self) -> u32 {
        45
    }

    fn run(&self, progress: &dyn ProgressCallback) -> Result<TestResult> {
        let chunk_size: usize = 64 * 1024; // 64KB chunks
        let num_threads = rayon::current_num_threads();
        let chunks_per_thread = 1000;
        let total_chunks = num_threads * chunks_per_thread;

        progress.update(0.0, &format!("Preparing {} threads...", num_threads));

        // Generate test data for each chunk
        let mut rng = rand::thread_rng();
        let chunks: Vec<Vec<u8>> = (0..total_chunks)
            .map(|_| {
                let mut data = vec![0u8; chunk_size];
                for i in 0..chunk_size {
                    data[i] = if i % 4 == 0 {
                        rng.gen()
                    } else {
                        data[i.saturating_sub(1)]
                    };
                }
                data
            })
            .collect();

        progress.update(0.1, "Warming up...");

        // Warmup
        chunks[..num_threads].par_iter().for_each(|data| {
            let compressed = compress_prepend_size(data);
            let _ = decompress_size_prepended(&compressed);
        });

        progress.update(0.2, "Running parallel benchmark...");

        let mut throughputs: Vec<f64> = Vec::new();
        let num_runs = 3;

        for run in 0..num_runs {
            if progress.is_cancelled() {
                return Err(anyhow::anyhow!("Cancelled"));
            }

            let bytes_processed = Arc::new(AtomicUsize::new(0));
            let bytes_clone = Arc::clone(&bytes_processed);

            let timer = Timer::new();

            chunks.par_iter().for_each(|data| {
                let compressed = compress_prepend_size(data);
                if let Ok(decompressed) = decompress_size_prepended(&compressed) {
                    bytes_clone.fetch_add(data.len() + decompressed.len(), Ordering::Relaxed);
                }
            });

            let elapsed = timer.elapsed_secs();
            let total_bytes = bytes_processed.load(Ordering::Relaxed);
            let mb_per_sec = (total_bytes as f64 / (1024.0 * 1024.0)) / elapsed;
            throughputs.push(mb_per_sec);

            progress.update(
                0.2 + (run as f32 / num_runs as f32) * 0.75,
                &format!("Run {}/{}: {:.0} MB/s", run + 1, num_runs, mb_per_sec),
            );
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

        let score = thresholds::multi_thread_score(median);

        progress.update(1.0, "Complete");

        Ok(TestResult {
            test_id: self.id().to_string(),
            name: self.name().to_string(),
            description: self.description().to_string(),
            value: median,
            unit: "MB/s".to_string(),
            score,
            max_score: 600,
            details: TestDetails {
                iterations: (total_chunks * num_runs) as u32,
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
