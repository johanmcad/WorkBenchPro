use anyhow::Result;
use rand::seq::SliceRandom;

use crate::benchmarks::{Benchmark, Category, ProgressCallback};
use crate::core::Timer;
use crate::models::{TestDetails, TestResult};

/// Memory latency benchmark using pointer-chasing
/// Measures nanoseconds per random memory access
pub struct MemoryLatencyBenchmark;

impl MemoryLatencyBenchmark {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MemoryLatencyBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl Benchmark for MemoryLatencyBenchmark {
    fn id(&self) -> &'static str {
        "memory_latency"
    }

    fn name(&self) -> &'static str {
        "Memory Latency"
    }

    fn description(&self) -> &'static str {
        "Pointer-chasing through large buffer - measures access latency"
    }

    fn category(&self) -> Category {
        Category::Responsiveness
    }

    fn estimated_duration_secs(&self) -> u32 {
        20
    }

    fn run(&self, progress: &dyn ProgressCallback) -> Result<TestResult> {
        // Use a buffer larger than typical L3 cache to measure main memory latency
        // 64MB buffer with 64-byte cache line sized elements
        let buffer_size: usize = 64 * 1024 * 1024;
        let element_size: usize = 64; // Cache line size
        let num_elements = buffer_size / element_size;
        let num_chases: usize = 10_000_000;

        progress.update(0.0, "Allocating memory buffer...");

        // Create buffer with pointer indices
        // Each element points to the next element in a random order
        let mut indices: Vec<usize> = (0..num_elements).collect();
        let mut rng = rand::thread_rng();
        indices.shuffle(&mut rng);

        // Create the pointer chain
        let mut chain: Vec<usize> = vec![0; num_elements];
        for i in 0..num_elements {
            let next_idx = (i + 1) % num_elements;
            chain[indices[i]] = indices[next_idx];
        }

        progress.update(0.1, "Warming up...");

        // Warmup: chase pointers to bring buffer into cache hierarchy
        let mut current = 0;
        for _ in 0..num_elements {
            current = chain[current];
        }

        progress.update(0.2, "Measuring memory latency...");

        let mut latencies: Vec<f64> = Vec::new();
        let num_runs = 5;

        for run in 0..num_runs {
            if progress.is_cancelled() {
                return Err(anyhow::anyhow!("Cancelled"));
            }

            let timer = Timer::new();

            // Chase pointers
            let mut idx = current;
            for _ in 0..num_chases {
                idx = chain[idx];
            }

            let elapsed_ns = timer.elapsed_secs() * 1_000_000_000.0;
            let latency_ns = elapsed_ns / num_chases as f64;
            latencies.push(latency_ns);

            // Use idx to prevent optimization
            current = idx;

            progress.update(
                0.2 + (run as f32 / num_runs as f32) * 0.75,
                &format!("Run {}/{}: {:.1} ns/access", run + 1, num_runs, latency_ns),
            );
        }

        // Calculate statistics
        latencies.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let min = latencies[0];
        let max = latencies[latencies.len() - 1];
        let sum: f64 = latencies.iter().sum();
        let mean = sum / latencies.len() as f64;
        let median = latencies[latencies.len() / 2];

        let variance: f64 = latencies
            .iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>()
            / latencies.len() as f64;
        let std_dev = variance.sqrt();

        progress.update(1.0, "Complete");

        Ok(TestResult {
            test_id: self.id().to_string(),
            name: self.name().to_string(),
            description: self.description().to_string(),
            value: median,
            unit: "ns".to_string(),
            details: TestDetails {
                iterations: (num_chases * num_runs) as u32,
                duration_secs: sum / 1_000_000_000.0 * num_chases as f64,
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
