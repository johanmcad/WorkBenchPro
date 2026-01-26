use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use anyhow::Result;

use crate::benchmarks::{Benchmark, Category, ProgressCallback};
use crate::core::Timer;
use crate::models::{TestDetails, TestResult};

/// Metadata operations benchmark - simulates npm install, build temp files
pub struct MetadataOpsBenchmark {
    test_dir: PathBuf,
}

impl MetadataOpsBenchmark {
    pub fn new() -> Self {
        Self {
            test_dir: std::env::temp_dir().join("workbench_metadata_ops"),
        }
    }

    fn cleanup(&self) {
        let _ = fs::remove_dir_all(&self.test_dir);
    }
}

impl Default for MetadataOpsBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl Benchmark for MetadataOpsBenchmark {
    fn id(&self) -> &'static str {
        "metadata_ops"
    }

    fn name(&self) -> &'static str {
        "Metadata Operations"
    }

    fn description(&self) -> &'static str {
        "Create/write/close/delete 5,000 small files - simulates build temp files, npm install"
    }

    fn category(&self) -> Category {
        Category::ProjectOperations
    }

    fn estimated_duration_secs(&self) -> u32 {
        30
    }

    fn run(&self, progress: &dyn ProgressCallback) -> Result<TestResult> {
        // Clean up any previous run
        self.cleanup();
        fs::create_dir_all(&self.test_dir)?;

        progress.update(0.0, "Running metadata operations...");

        let num_files = 5000;
        let num_runs = 3;
        let mut ops_per_sec_samples: Vec<f64> = Vec::with_capacity(num_runs);

        for run in 0..num_runs {
            if progress.is_cancelled() {
                self.cleanup();
                return Err(anyhow::anyhow!("Cancelled"));
            }

            let run_dir = self.test_dir.join(format!("run_{}", run));
            fs::create_dir_all(&run_dir)?;

            let timer = Timer::new();

            // Create, write, and delete files
            for i in 0..num_files {
                let file_path = run_dir.join(format!("file_{:05}.tmp", i));

                // Create and write
                let mut file = File::create(&file_path)?;
                writeln!(file, "Temporary file content {}", i)?;
                file.sync_all()?;
                drop(file);

                // Delete
                fs::remove_file(&file_path)?;

                if i % 500 == 0 {
                    progress.update(
                        (run as f32 / num_runs as f32)
                            + (i as f32 / num_files as f32) / num_runs as f32,
                        &format!("Run {}/{}, file {}/{}", run + 1, num_runs, i, num_files),
                    );
                }
            }

            let duration_secs = timer.elapsed_secs();
            // Each file has 3 operations: create+write, sync, delete
            let ops = num_files as f64 * 3.0;
            ops_per_sec_samples.push(ops / duration_secs);

            // Clean up run directory
            let _ = fs::remove_dir_all(&run_dir);
        }

        // Cleanup
        progress.update(0.9, "Cleaning up...");
        self.cleanup();

        // Calculate statistics
        ops_per_sec_samples.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let min = ops_per_sec_samples[0];
        let max = ops_per_sec_samples[ops_per_sec_samples.len() - 1];
        let sum: f64 = ops_per_sec_samples.iter().sum();
        let mean = sum / ops_per_sec_samples.len() as f64;
        let median = ops_per_sec_samples[ops_per_sec_samples.len() / 2];

        let variance: f64 = ops_per_sec_samples
            .iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>()
            / ops_per_sec_samples.len() as f64;
        let std_dev = variance.sqrt();

        progress.update(1.0, "Complete");

        Ok(TestResult {
            test_id: self.id().to_string(),
            name: self.name().to_string(),
            description: self.description().to_string(),
            value: median,
            unit: "ops/sec".to_string(),
            details: TestDetails {
                iterations: num_runs as u32,
                duration_secs: sum,
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
