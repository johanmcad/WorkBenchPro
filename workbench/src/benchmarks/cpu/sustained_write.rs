use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use anyhow::Result;
use rand::Rng;

use crate::benchmarks::{Benchmark, Category, ProgressCallback};
use crate::core::Timer;
use crate::models::{TestDetails, TestResult};

/// Sustained write performance benchmark
/// Simulates build output - writing large amounts of data with periodic fsync
pub struct SustainedWriteBenchmark {
    test_file: PathBuf,
}

impl SustainedWriteBenchmark {
    pub fn new() -> Self {
        Self {
            test_file: std::env::temp_dir().join("workbench_pro_sustained_write.bin"),
        }
    }

    fn cleanup(&self) {
        let _ = fs::remove_file(&self.test_file);
    }
}

impl Default for SustainedWriteBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl Benchmark for SustainedWriteBenchmark {
    fn id(&self) -> &'static str {
        "sustained_write"
    }

    fn name(&self) -> &'static str {
        "Sustained Write Performance"
    }

    fn description(&self) -> &'static str {
        "Write 4GB with periodic fsync - simulates build output"
    }

    fn category(&self) -> Category {
        Category::BuildPerformance
    }

    fn estimated_duration_secs(&self) -> u32 {
        60
    }

    fn is_synthetic(&self) -> bool {
        true
    }

    fn run(&self, progress: &dyn ProgressCallback) -> Result<TestResult> {
        let total_size: u64 = 4 * 1024 * 1024 * 1024; // 4GB
        let chunk_size: usize = 4 * 1024 * 1024; // 4MB chunks
        let fsync_interval = 64; // fsync every 64 chunks (256MB)
        let num_chunks = (total_size / chunk_size as u64) as usize;

        progress.update(0.0, "Preparing write buffer...");

        // Generate random data
        let mut rng = rand::thread_rng();
        let mut buffer = vec![0u8; chunk_size];
        rng.fill(&mut buffer[..]);

        progress.update(0.05, "Starting sustained write...");

        let mut throughputs: Vec<f64> = Vec::new();
        let num_runs = 2; // Fewer runs due to large data size

        for run in 0..num_runs {
            if progress.is_cancelled() {
                self.cleanup();
                return Err(anyhow::anyhow!("Cancelled"));
            }

            // Remove existing file
            self.cleanup();

            let mut file = File::create(&self.test_file)?;
            let timer = Timer::new();
            let mut bytes_written: u64 = 0;

            for i in 0..num_chunks {
                if progress.is_cancelled() {
                    self.cleanup();
                    return Err(anyhow::anyhow!("Cancelled"));
                }

                file.write_all(&buffer)?;
                bytes_written += chunk_size as u64;

                // Periodic fsync to simulate realistic build output
                if (i + 1) % fsync_interval == 0 {
                    file.sync_data()?;
                }

                if i % 64 == 0 {
                    let run_progress = 0.05 + (run as f32 / num_runs as f32) * 0.9
                        + (i as f32 / num_chunks as f32) * (0.9 / num_runs as f32);
                    progress.update(
                        run_progress,
                        &format!(
                            "Run {}/{}: Writing {}MB / {}MB",
                            run + 1,
                            num_runs,
                            bytes_written / (1024 * 1024),
                            total_size / (1024 * 1024)
                        ),
                    );
                }
            }

            // Final sync
            file.sync_all()?;

            let elapsed = timer.elapsed_secs();
            let mb_per_sec = (bytes_written as f64 / (1024.0 * 1024.0)) / elapsed;
            throughputs.push(mb_per_sec);
        }

        // Cleanup
        progress.update(0.95, "Cleaning up...");
        self.cleanup();

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
                iterations: num_runs as u32,
                duration_secs: (total_size as f64 * num_runs as f64) / (mean * 1024.0 * 1024.0),
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
