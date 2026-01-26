use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;

use anyhow::Result;
use lz4_flex::compress_prepend_size;
use rand::Rng;
use rayon::prelude::*;

use crate::benchmarks::{Benchmark, Category, ProgressCallback};
use crate::core::Timer;
use crate::models::{TestDetails, TestResult};

/// Mixed read-compute-write benchmark
/// Simulates full build cycle: read source -> compile -> write output
pub struct MixedWorkloadBenchmark {
    test_dir: PathBuf,
}

impl MixedWorkloadBenchmark {
    pub fn new() -> Self {
        Self {
            test_dir: std::env::temp_dir().join("workbench_pro_mixed_test"),
        }
    }

    fn setup(&self, progress: &dyn ProgressCallback) -> Result<Vec<PathBuf>> {
        progress.update(0.0, "Creating test files...");

        fs::create_dir_all(&self.test_dir)?;

        let num_files = 500;
        let file_size = 64 * 1024; // 64KB per file
        let mut rng = rand::thread_rng();
        let mut files = Vec::with_capacity(num_files);

        for i in 0..num_files {
            if progress.is_cancelled() {
                return Err(anyhow::anyhow!("Cancelled"));
            }

            let path = self.test_dir.join(format!("input_{:04}.dat", i));
            let mut file = File::create(&path)?;

            // Generate compressible data
            let mut data = vec![0u8; file_size];
            for j in 0..file_size {
                data[j] = if j % 4 == 0 {
                    rng.gen()
                } else {
                    data[j.saturating_sub(1)]
                };
            }

            file.write_all(&data)?;
            files.push(path);

            if i % 50 == 0 {
                progress.update(
                    (i as f32 / num_files as f32) * 0.2,
                    &format!("Creating file {}/{}...", i, num_files),
                );
            }
        }

        Ok(files)
    }

    fn cleanup(&self) {
        let _ = fs::remove_dir_all(&self.test_dir);
    }
}

impl Default for MixedWorkloadBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl Benchmark for MixedWorkloadBenchmark {
    fn id(&self) -> &'static str {
        "mixed_workload"
    }

    fn name(&self) -> &'static str {
        "Mixed Read-Compute-Write"
    }

    fn description(&self) -> &'static str {
        "Read files, compress, write output - simulates full build cycle"
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
        // Setup
        let input_files = self.setup(progress)?;

        progress.update(0.2, "Running mixed workload...");

        let mut throughputs: Vec<f64> = Vec::new();
        let num_runs = 3;

        for run in 0..num_runs {
            if progress.is_cancelled() {
                self.cleanup();
                return Err(anyhow::anyhow!("Cancelled"));
            }

            // Clean output directory
            let output_dir = self.test_dir.join(format!("output_{}", run));
            fs::create_dir_all(&output_dir)?;

            let timer = Timer::new();

            // Process all files in parallel: read -> compress -> write
            let results: Vec<std::io::Result<usize>> = input_files
                .par_iter()
                .map(|input_path| {
                    // Read
                    let mut file = File::open(input_path)?;
                    let mut data = Vec::new();
                    file.read_to_end(&mut data)?;

                    // Compute (compress)
                    let compressed = compress_prepend_size(&data);

                    // Write
                    let output_name = input_path
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .replace(".dat", ".lz4");
                    let output_path = output_dir.join(output_name);
                    let mut out_file = File::create(&output_path)?;
                    out_file.write_all(&compressed)?;

                    Ok(data.len() + compressed.len())
                })
                .collect();

            let elapsed = timer.elapsed_secs();

            // Sum up bytes processed
            let total_bytes: usize = results
                .into_iter()
                .filter_map(|r| r.ok())
                .sum();

            let mb_per_sec = (total_bytes as f64 / (1024.0 * 1024.0)) / elapsed;
            throughputs.push(mb_per_sec);

            // Cleanup output
            let _ = fs::remove_dir_all(&output_dir);

            progress.update(
                0.2 + (run as f32 / num_runs as f32) * 0.7,
                &format!("Run {}/{}: {:.0} MB/s", run + 1, num_runs, mb_per_sec),
            );
        }

        // Cleanup
        progress.update(0.9, "Cleaning up...");
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
                iterations: (input_files.len() * num_runs) as u32,
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
