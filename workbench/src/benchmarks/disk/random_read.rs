use std::fs::{self, File};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

use anyhow::Result;
use rand::Rng;

use crate::benchmarks::{Benchmark, Category, ProgressCallback};
use crate::core::Timer;
use crate::models::{Percentiles, TestDetails, TestResult};

/// Small file random read benchmark - simulates loading source files
pub struct RandomReadBenchmark {
    test_file: PathBuf,
}

impl RandomReadBenchmark {
    pub fn new() -> Self {
        Self {
            test_file: std::env::temp_dir().join("workbench_pro_random_read.bin"),
        }
    }

    fn setup(&self, progress: &dyn ProgressCallback) -> Result<()> {
        progress.update(0.0, "Creating test file (1GB)...");

        // Create 1GB file
        let file_size: u64 = 1024 * 1024 * 1024; // 1GB
        let chunk_size: usize = 1024 * 1024; // 1MB chunks
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

            if i % 100 == 0 {
                progress.update(
                    (i as f32 / num_chunks as f32) * 0.3,
                    &format!("Creating test file... {}MB", i),
                );
            }
        }

        file.sync_all()?;
        Ok(())
    }

    fn cleanup(&self) {
        let _ = fs::remove_file(&self.test_file);
    }
}

impl Default for RandomReadBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl Benchmark for RandomReadBenchmark {
    fn id(&self) -> &'static str {
        "random_read"
    }

    fn name(&self) -> &'static str {
        "Small File Random Read"
    }

    fn description(&self) -> &'static str {
        "10,000 random 4KB reads from 1GB file - simulates loading source files"
    }

    fn category(&self) -> Category {
        Category::ProjectOperations
    }

    fn estimated_duration_secs(&self) -> u32 {
        60
    }

    fn is_synthetic(&self) -> bool {
        true
    }

    fn run(&self, progress: &dyn ProgressCallback) -> Result<TestResult> {
        // Setup
        self.setup(progress)?;

        progress.update(0.3, "Running random read tests...");

        let file_size: u64 = 1024 * 1024 * 1024;
        let read_size: usize = 4096; // 4KB
        let num_reads = 10_000;
        let max_offset = file_size - read_size as u64;

        let mut file = File::open(&self.test_file)?;
        let mut buffer = vec![0u8; read_size];
        let mut rng = rand::thread_rng();
        let mut latencies_ms: Vec<f64> = Vec::with_capacity(num_reads);

        // Generate random offsets
        let offsets: Vec<u64> = (0..num_reads)
            .map(|_| rng.gen_range(0..max_offset))
            .collect();

        let total_timer = Timer::new();

        for (i, &offset) in offsets.iter().enumerate() {
            if progress.is_cancelled() {
                self.cleanup();
                return Err(anyhow::anyhow!("Cancelled"));
            }

            let timer = Timer::new();
            file.seek(SeekFrom::Start(offset))?;
            file.read_exact(&mut buffer)?;
            latencies_ms.push(timer.elapsed_ms());

            if i % 1000 == 0 {
                progress.update(
                    0.3 + (i as f32 / num_reads as f32) * 0.6,
                    &format!("Reading... {}/{}", i, num_reads),
                );
            }
        }

        let total_duration = total_timer.elapsed_secs();

        // Cleanup
        progress.update(0.9, "Cleaning up...");
        self.cleanup();

        // Calculate statistics
        latencies_ms.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let min = latencies_ms[0];
        let max = latencies_ms[latencies_ms.len() - 1];
        let sum: f64 = latencies_ms.iter().sum();
        let mean = sum / latencies_ms.len() as f64;
        let median = latencies_ms[latencies_ms.len() / 2];

        let variance: f64 = latencies_ms.iter().map(|x| (x - mean).powi(2)).sum::<f64>()
            / latencies_ms.len() as f64;
        let std_dev = variance.sqrt();

        let percentiles = Percentiles::from_sorted_values(&latencies_ms);
        let p99 = percentiles.p99;

        progress.update(1.0, "Complete");

        Ok(TestResult {
            test_id: self.id().to_string(),
            name: self.name().to_string(),
            description: self.description().to_string(),
            value: p99,
            unit: "ms (P99)".to_string(),
            details: TestDetails {
                iterations: num_reads as u32,
                duration_secs: total_duration,
                min,
                max,
                mean,
                median,
                std_dev,
                percentiles: Some(percentiles),
            },
        })
    }
}
