use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

use anyhow::Result;
use rand::Rng;

use crate::benchmarks::{Benchmark, Category, ProgressCallback};
use crate::core::Timer;
use crate::models::{Percentiles, TestDetails, TestResult};

/// Storage latency distribution benchmark
/// Measures P50, P95, P99, P99.9 latency for random 4KB reads
pub struct StorageLatencyBenchmark {
    test_file: PathBuf,
}

impl StorageLatencyBenchmark {
    pub fn new() -> Self {
        Self {
            test_file: std::env::temp_dir().join("workbench_latency_test.bin"),
        }
    }

    fn setup(&self, progress: &dyn ProgressCallback) -> Result<()> {
        progress.update(0.0, "Creating test file (1GB)...");

        let file_size: u64 = 1024 * 1024 * 1024; // 1GB
        let chunk_size: usize = 4 * 1024 * 1024; // 4MB chunks
        let num_chunks = (file_size / chunk_size as u64) as usize;

        let mut file = File::create(&self.test_file)?;
        let mut rng = rand::thread_rng();
        let mut buffer = vec![0u8; chunk_size];

        for i in 0..num_chunks {
            if progress.is_cancelled() {
                return Err(anyhow::anyhow!("Cancelled"));
            }

            rng.fill(&mut buffer[..]);
            file.write_all(&buffer)?;

            if i % 25 == 0 {
                progress.update(
                    (i as f32 / num_chunks as f32) * 0.3,
                    &format!("Creating test file... {}MB", (i * chunk_size) / (1024 * 1024)),
                );
            }
        }

        file.sync_all()?;
        Ok(())
    }

    fn cleanup(&self) {
        let _ = std::fs::remove_file(&self.test_file);
    }
}

impl Default for StorageLatencyBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl Benchmark for StorageLatencyBenchmark {
    fn id(&self) -> &'static str {
        "storage_latency"
    }

    fn name(&self) -> &'static str {
        "Storage Latency Distribution"
    }

    fn description(&self) -> &'static str {
        "10,000 random 4KB reads measuring P50/P95/P99/P99.9 latency"
    }

    fn category(&self) -> Category {
        Category::Responsiveness
    }

    fn estimated_duration_secs(&self) -> u32 {
        120
    }

    fn run(&self, progress: &dyn ProgressCallback) -> Result<TestResult> {
        // Setup
        self.setup(progress)?;

        let file_size: u64 = 1024 * 1024 * 1024;
        let read_size: usize = 4096; // 4KB
        let num_reads: usize = 10000;
        let max_offset = file_size - read_size as u64;

        progress.update(0.3, "Running latency tests...");

        let mut file = File::open(&self.test_file)?;
        let mut buffer = vec![0u8; read_size];
        let mut rng = rand::thread_rng();
        let mut latencies_us: Vec<f64> = Vec::with_capacity(num_reads);

        // Warmup
        for _ in 0..100 {
            let offset = rng.gen_range(0..max_offset);
            file.seek(SeekFrom::Start(offset))?;
            file.read_exact(&mut buffer)?;
        }

        // Actual measurements
        for i in 0..num_reads {
            if progress.is_cancelled() {
                self.cleanup();
                return Err(anyhow::anyhow!("Cancelled"));
            }

            let offset = rng.gen_range(0..max_offset);

            let timer = Timer::new();
            file.seek(SeekFrom::Start(offset))?;
            file.read_exact(&mut buffer)?;
            let elapsed_us = timer.elapsed_secs() * 1_000_000.0;

            latencies_us.push(elapsed_us);

            if i % 1000 == 0 {
                progress.update(
                    0.3 + (i as f32 / num_reads as f32) * 0.6,
                    &format!("Measuring latency... {}/{}", i, num_reads),
                );
            }
        }

        // Cleanup
        progress.update(0.9, "Cleaning up...");
        self.cleanup();

        // Calculate percentiles
        latencies_us.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let p50_idx = (num_reads as f64 * 0.50) as usize;
        let p75_idx = (num_reads as f64 * 0.75) as usize;
        let p90_idx = (num_reads as f64 * 0.90) as usize;
        let p95_idx = (num_reads as f64 * 0.95) as usize;
        let p99_idx = (num_reads as f64 * 0.99) as usize;
        let p999_idx = (num_reads as f64 * 0.999) as usize;

        let p50 = latencies_us[p50_idx];
        let p75 = latencies_us[p75_idx];
        let p90 = latencies_us[p90_idx];
        let p95 = latencies_us[p95_idx];
        let p99 = latencies_us[p99_idx];
        let p999 = latencies_us[p999_idx.min(num_reads - 1)];

        let min = latencies_us[0];
        let max = latencies_us[num_reads - 1];
        let sum: f64 = latencies_us.iter().sum();
        let mean = sum / num_reads as f64;
        let median = latencies_us[num_reads / 2];

        let variance: f64 = latencies_us
            .iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>()
            / num_reads as f64;
        let std_dev = variance.sqrt();

        // Convert P99 to ms for display
        let p99_ms = p99 / 1000.0;

        progress.update(1.0, "Complete");

        Ok(TestResult {
            test_id: self.id().to_string(),
            name: self.name().to_string(),
            description: self.description().to_string(),
            value: p99_ms,
            unit: "ms (P99)".to_string(),
            details: TestDetails {
                iterations: num_reads as u32,
                duration_secs: sum / 1_000_000.0,
                min: min / 1000.0, // Convert to ms
                max: max / 1000.0,
                mean: mean / 1000.0,
                median: median / 1000.0,
                std_dev: std_dev / 1000.0,
                percentiles: Some(Percentiles {
                    p50: p50 / 1000.0,
                    p75: p75 / 1000.0,
                    p90: p90 / 1000.0,
                    p95: p95 / 1000.0,
                    p99: p99 / 1000.0,
                    p999: p999 / 1000.0,
                }),
            },
        })
    }
}
