use std::fs;
use std::io::Write;
use std::path::PathBuf;

use anyhow::Result;

use crate::benchmarks::{Benchmark, Category, ProgressCallback};
use crate::core::Timer;
use crate::models::{Percentiles, TestDetails, TestResult};
use crate::scoring::thresholds;

/// File enumeration benchmark - simulates VS solution load, git status
pub struct FileEnumerationBenchmark {
    test_dir: PathBuf,
}

impl FileEnumerationBenchmark {
    pub fn new() -> Self {
        Self {
            test_dir: std::env::temp_dir().join("workbench_file_enum"),
        }
    }

    fn setup(&self, progress: &dyn ProgressCallback) -> Result<()> {
        progress.update(0.0, "Setting up test files...");

        // Clean up any previous run
        if self.test_dir.exists() {
            fs::remove_dir_all(&self.test_dir)?;
        }

        fs::create_dir_all(&self.test_dir)?;

        // Create 30,000 files in 500 directories
        let num_dirs = 500;
        let files_per_dir = 60;
        let total_files = num_dirs * files_per_dir;

        for dir_idx in 0..num_dirs {
            if progress.is_cancelled() {
                return Err(anyhow::anyhow!("Cancelled"));
            }

            let dir_path = self.test_dir.join(format!("dir_{:04}", dir_idx));
            fs::create_dir_all(&dir_path)?;

            for file_idx in 0..files_per_dir {
                let file_path = dir_path.join(format!("file_{:04}.txt", file_idx));
                let mut file = fs::File::create(&file_path)?;
                // Write some content to make it realistic
                writeln!(file, "Test file content for benchmarking")?;
            }

            if dir_idx % 50 == 0 {
                progress.update(
                    (dir_idx as f32 / num_dirs as f32) * 0.5,
                    &format!("Creating files... {}/{}", dir_idx * files_per_dir, total_files),
                );
            }
        }

        Ok(())
    }

    fn cleanup(&self) {
        let _ = fs::remove_dir_all(&self.test_dir);
    }

    fn run_enumeration(&self) -> Result<u64> {
        let mut count = 0u64;

        fn count_files(path: &PathBuf, count: &mut u64) -> Result<()> {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    count_files(&path, count)?;
                } else {
                    *count += 1;
                }
            }
            Ok(())
        }

        count_files(&self.test_dir, &mut count)?;
        Ok(count)
    }
}

impl Default for FileEnumerationBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl Benchmark for FileEnumerationBenchmark {
    fn id(&self) -> &'static str {
        "file_enumeration"
    }

    fn name(&self) -> &'static str {
        "File Enumeration"
    }

    fn description(&self) -> &'static str {
        "Enumerate 30,000 files in 500 directories - simulates VS solution load, git status"
    }

    fn category(&self) -> Category {
        Category::ProjectOperations
    }

    fn estimated_duration_secs(&self) -> u32 {
        30
    }

    fn run(&self, progress: &dyn ProgressCallback) -> Result<TestResult> {
        // Setup
        self.setup(progress)?;

        progress.update(0.5, "Running enumeration tests...");

        // Warmup run
        let _ = self.run_enumeration()?;

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
            files_counted = self.run_enumeration()?;
            let duration_ms = timer.elapsed_ms();
            durations_ms.push(duration_ms);

            progress.update(
                0.5 + (run_idx as f32 / num_runs as f32) * 0.4,
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

        // Calculate score
        let score = thresholds::file_enumeration_score(files_per_sec);

        progress.update(1.0, "Complete");

        Ok(TestResult {
            test_id: self.id().to_string(),
            name: self.name().to_string(),
            description: self.description().to_string(),
            value: files_per_sec,
            unit: "files/sec".to_string(),
            score,
            max_score: 500,
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
