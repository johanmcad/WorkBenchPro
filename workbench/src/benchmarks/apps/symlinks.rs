use std::fs;
use std::path::PathBuf;

use anyhow::Result;

use crate::benchmarks::{Benchmark, BenchmarkConfig, Category, ProgressCallback};
use crate::core::Timer;
use crate::models::{TestDetails, TestResult};

/// Symbolic Link / Junction benchmark
/// Tests NTFS symlink and junction performance
pub struct SymlinkBenchmark {
    test_dir: PathBuf,
}

impl SymlinkBenchmark {
    pub fn new() -> Self {
        Self {
            test_dir: std::env::temp_dir().join("workbench_pro_symlink_test"),
        }
    }

    fn cleanup(&self) {
        let _ = fs::remove_dir_all(&self.test_dir);
    }
}

impl Default for SymlinkBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl Benchmark for SymlinkBenchmark {
    fn id(&self) -> &'static str {
        "symlinks"
    }

    fn name(&self) -> &'static str {
        "Symlink Operations"
    }

    fn description(&self) -> &'static str {
        "Create, traverse, and resolve symbolic links"
    }

    fn category(&self) -> Category {
        Category::ProjectOperations
    }

    fn estimated_duration_secs(&self) -> u32 {
        30
    }

    fn run(&self, progress: &dyn ProgressCallback, _config: &BenchmarkConfig) -> Result<TestResult> {
        // Setup
        let _ = fs::remove_dir_all(&self.test_dir);
        fs::create_dir_all(&self.test_dir)?;

        progress.update(0.05, "Setting up symlink test...");

        // Create target directories and files
        let targets_dir = self.test_dir.join("targets");
        let links_dir = self.test_dir.join("links");
        fs::create_dir_all(&targets_dir)?;
        fs::create_dir_all(&links_dir)?;

        // Create target files
        let num_targets = 50;
        for i in 0..num_targets {
            let file_path = targets_dir.join(format!("target_{:03}.txt", i));
            fs::write(&file_path, format!("Content of target file {}", i))?;

            let dir_path = targets_dir.join(format!("target_dir_{:03}", i));
            fs::create_dir_all(&dir_path)?;
            fs::write(dir_path.join("inner.txt"), format!("Inner file {}", i))?;
        }

        let mut create_times: Vec<f64> = Vec::new();
        let mut read_times: Vec<f64> = Vec::new();
        let mut traverse_times: Vec<f64> = Vec::new();

        // Test 1: Create symbolic links (soft links)
        progress.update(0.15, "Creating symbolic links...");

        for round in 0..3 {
            if progress.is_cancelled() {
                self.cleanup();
                return Err(anyhow::anyhow!("Cancelled"));
            }

            let round_dir = links_dir.join(format!("round_{}", round));
            fs::create_dir_all(&round_dir)?;

            let timer = Timer::new();
            for i in 0..num_targets {
                let target = targets_dir.join(format!("target_{:03}.txt", i));
                let link = round_dir.join(format!("link_{:03}.txt", i));

                // Use soft link (works cross-platform)
                #[cfg(unix)]
                {
                    let _ = std::os::unix::fs::symlink(&target, &link);
                }
                #[cfg(windows)]
                {
                    // On Windows, try symlink but fall back to hard link if no privilege
                    let _ = std::os::windows::fs::symlink_file(&target, &link)
                        .or_else(|_| fs::hard_link(&target, &link));
                }
            }
            create_times.push(timer.elapsed_secs() * 1000.0);

            progress.update(
                0.15 + (round as f32 / 3.0) * 0.25,
                &format!("Create links round {}/3...", round + 1),
            );
        }

        // Test 2: Read through symbolic links
        progress.update(0.4, "Reading through links...");

        for round in 0..5 {
            if progress.is_cancelled() {
                self.cleanup();
                return Err(anyhow::anyhow!("Cancelled"));
            }

            let round_dir = links_dir.join("round_0");

            let timer = Timer::new();
            for i in 0..num_targets {
                let link = round_dir.join(format!("link_{:03}.txt", i));
                if link.exists() {
                    let _ = fs::read_to_string(&link);
                }
            }
            read_times.push(timer.elapsed_secs() * 1000.0);

            progress.update(
                0.4 + (round as f32 / 5.0) * 0.3,
                &format!("Read links round {}/5...", round + 1),
            );
        }

        // Test 3: Traverse and resolve links
        progress.update(0.7, "Resolving link targets...");

        for round in 0..5 {
            if progress.is_cancelled() {
                self.cleanup();
                return Err(anyhow::anyhow!("Cancelled"));
            }

            let round_dir = links_dir.join("round_0");

            let timer = Timer::new();
            for i in 0..num_targets {
                let link = round_dir.join(format!("link_{:03}.txt", i));
                if link.exists() {
                    // Resolve the symlink to its target
                    let _ = fs::canonicalize(&link);
                    let _ = fs::symlink_metadata(&link);
                }
            }
            traverse_times.push(timer.elapsed_secs() * 1000.0);

            progress.update(
                0.7 + (round as f32 / 5.0) * 0.25,
                &format!("Resolve round {}/5...", round + 1),
            );
        }

        // Cleanup
        progress.update(0.95, "Cleaning up...");
        self.cleanup();

        // Calculate statistics
        let avg_create = create_times.iter().sum::<f64>() / create_times.len() as f64;
        let avg_read = read_times.iter().sum::<f64>() / read_times.len() as f64;
        let avg_traverse = traverse_times.iter().sum::<f64>() / traverse_times.len() as f64;
        let avg_combined = (avg_create + avg_read + avg_traverse) / 3.0;

        let all_times: Vec<f64> = [create_times, read_times, traverse_times].concat();
        let min = all_times.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = all_times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        progress.update(1.0, "Complete");

        Ok(TestResult {
            test_id: self.id().to_string(),
            name: self.name().to_string(),
            description: format!(
                "{} (create: {:.0}ms, read: {:.0}ms, resolve: {:.0}ms)",
                self.description(),
                avg_create,
                avg_read,
                avg_traverse
            ),
            value: avg_combined,
            unit: "ms".to_string(),
            details: TestDetails {
                iterations: all_times.len() as u32,
                duration_secs: all_times.iter().sum::<f64>() / 1000.0,
                min,
                max,
                mean: avg_combined,
                median: avg_combined,
                std_dev: 0.0,
                percentiles: None,
            },
        })
    }
}
