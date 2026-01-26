use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;

use anyhow::Result;

use crate::benchmarks::{Benchmark, Category, ProgressCallback};
use crate::core::Timer;
use crate::models::{Percentiles, TestDetails, TestResult};

/// Directory traversal with content benchmark - simulates search in files
pub struct TraversalBenchmark {
    test_dir: PathBuf,
}

impl TraversalBenchmark {
    pub fn new() -> Self {
        Self {
            test_dir: std::env::temp_dir().join("workbench_traversal"),
        }
    }

    fn setup(&self, progress: &dyn ProgressCallback) -> Result<()> {
        progress.update(0.0, "Setting up test files...");

        // Clean up any previous run
        if self.test_dir.exists() {
            fs::remove_dir_all(&self.test_dir)?;
        }

        fs::create_dir_all(&self.test_dir)?;

        // Create 30,000 files in 500 directories with content
        let num_dirs = 500;
        let files_per_dir = 60;
        let total_files = num_dirs * files_per_dir;

        // Create some realistic content
        let content = "// This is a source file for benchmarking\nfn main() {\n    println!(\"Hello, World!\");\n}\n";

        for dir_idx in 0..num_dirs {
            if progress.is_cancelled() {
                return Err(anyhow::anyhow!("Cancelled"));
            }

            let dir_path = self.test_dir.join(format!("src_{:04}", dir_idx));
            fs::create_dir_all(&dir_path)?;

            for file_idx in 0..files_per_dir {
                let file_path = dir_path.join(format!("module_{:04}.rs", file_idx));
                let mut file = File::create(&file_path)?;
                file.write_all(content.as_bytes())?;
            }

            if dir_idx % 50 == 0 {
                progress.update(
                    (dir_idx as f32 / num_dirs as f32) * 0.4,
                    &format!("Creating files... {}/{}", dir_idx * files_per_dir, total_files),
                );
            }
        }

        Ok(())
    }

    fn cleanup(&self) {
        let _ = fs::remove_dir_all(&self.test_dir);
    }

    fn run_traversal(&self) -> Result<u64> {
        let mut count = 0u64;
        let mut buffer = vec![0u8; 1024]; // Read first 1KB

        fn traverse(path: &PathBuf, count: &mut u64, buffer: &mut [u8]) -> Result<()> {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    traverse(&path, count, buffer)?;
                } else {
                    // Read first 1KB of each file
                    let mut file = File::open(&path)?;
                    let _ = file.read(buffer)?;
                    *count += 1;
                }
            }
            Ok(())
        }

        traverse(&self.test_dir, &mut count, &mut buffer)?;
        Ok(count)
    }
}

impl Default for TraversalBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl Benchmark for TraversalBenchmark {
    fn id(&self) -> &'static str {
        "dir_traversal"
    }

    fn name(&self) -> &'static str {
        "Directory Traversal with Content"
    }

    fn description(&self) -> &'static str {
        "Enumerate + read first 1KB of 30,000 files - simulates search in files"
    }

    fn category(&self) -> Category {
        Category::ProjectOperations
    }

    fn estimated_duration_secs(&self) -> u32 {
        45
    }

    fn run(&self, progress: &dyn ProgressCallback) -> Result<TestResult> {
        // Setup
        self.setup(progress)?;

        progress.update(0.4, "Running traversal tests...");

        // Warmup run
        let _ = self.run_traversal()?;

        // Actual runs
        let num_runs = 5;
        let mut durations_ms: Vec<f64> = Vec::with_capacity(num_runs);
        let mut files_counted = 0u64;

        for run_idx in 0..num_runs {
            if progress.is_cancelled() {
                self.cleanup();
                return Err(anyhow::anyhow!("Cancelled"));
            }

            let timer = Timer::new();
            files_counted = self.run_traversal()?;
            let duration_ms = timer.elapsed_ms();
            durations_ms.push(duration_ms);

            progress.update(
                0.4 + (run_idx as f32 / num_runs as f32) * 0.5,
                &format!("Run {}/{}", run_idx + 1, num_runs),
            );
        }

        // Cleanup
        progress.update(0.9, "Cleaning up...");
        self.cleanup();

        // Calculate statistics
        durations_ms.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let min = durations_ms[0];
        let max = durations_ms[durations_ms.len() - 1];
        let sum: f64 = durations_ms.iter().sum();
        let mean = sum / durations_ms.len() as f64;
        let median = durations_ms[durations_ms.len() / 2];

        let variance: f64 = durations_ms.iter().map(|x| (x - mean).powi(2)).sum::<f64>()
            / durations_ms.len() as f64;
        let std_dev = variance.sqrt();

        // Calculate files per second (using median)
        let files_per_sec = (files_counted as f64 / median) * 1000.0;

        progress.update(1.0, "Complete");

        Ok(TestResult {
            test_id: self.id().to_string(),
            name: self.name().to_string(),
            description: self.description().to_string(),
            value: files_per_sec,
            unit: "files/sec".to_string(),
            details: TestDetails {
                iterations: num_runs as u32,
                duration_secs: sum / 1000.0,
                min,
                max,
                mean,
                median,
                std_dev,
                percentiles: Some(Percentiles::from_sorted_values(&durations_ms)),
            },
        })
    }
}
