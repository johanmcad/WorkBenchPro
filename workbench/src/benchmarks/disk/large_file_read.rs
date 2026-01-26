use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;

use anyhow::Result;
use rand::Rng;

use crate::benchmarks::{Benchmark, BenchmarkConfig, Category, ProgressCallback};
use crate::core::Timer;
use crate::models::{TestDetails, TestResult};

/// Large file sequential read benchmark - simulates opening large CAD files
pub struct LargeFileReadBenchmark {
    test_file: PathBuf,
}

impl LargeFileReadBenchmark {
    pub fn new() -> Self {
        Self {
            test_file: std::env::temp_dir().join("workbench_pro_large_file.bin"),
        }
    }

    fn setup_with_size(&self, progress: &dyn ProgressCallback, size_mb: u32) -> Result<()> {
        progress.update(0.0, &format!("Creating test file ({}MB)...", size_mb));

        let file_size: u64 = size_mb as u64 * 1024 * 1024;
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

            if i % 50 == 0 {
                progress.update(
                    (i as f32 / num_chunks as f32) * 0.4,
                    &format!("Creating test file... {}MB", (i * chunk_size) / (1024 * 1024)),
                );
            }
        }

        file.sync_all()?;
        Ok(())
    }

    fn cleanup(&self) {
        let _ = fs::remove_file(&self.test_file);
    }

    fn run_read_with_size(&self, size_mb: u32) -> Result<f64> {
        let file_size: u64 = size_mb as u64 * 1024 * 1024;
        let chunk_size: usize = 1024 * 1024; // 1MB chunks

        let mut file = File::open(&self.test_file)?;
        let mut buffer = vec![0u8; chunk_size];

        let timer = Timer::new();

        loop {
            let bytes_read = file.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
        }

        let duration_secs = timer.elapsed_secs();
        let mb_per_sec = (file_size as f64 / (1024.0 * 1024.0)) / duration_secs;

        Ok(mb_per_sec)
    }
}

impl Default for LargeFileReadBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl Benchmark for LargeFileReadBenchmark {
    fn id(&self) -> &'static str {
        "large_file_read"
    }

    fn name(&self) -> &'static str {
        "Large File Sequential Read"
    }

    fn description(&self) -> &'static str {
        "Read 2GB file in 1MB chunks - simulates opening large CAD files"
    }

    fn category(&self) -> Category {
        Category::ProjectOperations
    }

    fn estimated_duration_secs(&self) -> u32 {
        90
    }

    fn is_synthetic(&self) -> bool {
        true
    }

    fn run(&self, progress: &dyn ProgressCallback, config: &BenchmarkConfig) -> Result<TestResult> {
        let file_size_mb = config.disk_large_file_mb;

        // Setup with configured file size
        self.setup_with_size(progress, file_size_mb)?;

        progress.update(0.4, "Running sequential read tests...");

        // Warmup run
        let _ = self.run_read_with_size(file_size_mb)?;

        // Actual runs
        let num_runs = config.iterations as usize;
        let mut speeds_mb_per_sec: Vec<f64> = Vec::with_capacity(num_runs);

        for run_idx in 0..num_runs {
            if progress.is_cancelled() {
                self.cleanup();
                return Err(anyhow::anyhow!("Cancelled"));
            }

            let mb_per_sec = self.run_read_with_size(file_size_mb)?;
            speeds_mb_per_sec.push(mb_per_sec);

            progress.update(
                0.4 + (run_idx as f32 / num_runs as f32) * 0.5,
                &format!("Run {}/{}: {:.0} MB/s", run_idx + 1, num_runs, mb_per_sec),
            );
        }

        // Cleanup
        progress.update(0.9, "Cleaning up...");
        self.cleanup();

        // Calculate statistics
        speeds_mb_per_sec.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let min = speeds_mb_per_sec[0];
        let max = speeds_mb_per_sec[speeds_mb_per_sec.len() - 1];
        let sum: f64 = speeds_mb_per_sec.iter().sum();
        let mean = sum / speeds_mb_per_sec.len() as f64;
        let median = speeds_mb_per_sec[speeds_mb_per_sec.len() / 2];

        let variance: f64 = speeds_mb_per_sec
            .iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>()
            / speeds_mb_per_sec.len() as f64;
        let std_dev = variance.sqrt();

        progress.update(1.0, "Complete");

        Ok(TestResult {
            test_id: self.id().to_string(),
            name: self.name().to_string(),
            description: format!("Read {}MB file in 1MB chunks", file_size_mb),
            value: median,
            unit: "MB/s".to_string(),
            details: TestDetails {
                iterations: num_runs as u32,
                duration_secs: (file_size_mb as f64 * num_runs as f64) / mean,
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
