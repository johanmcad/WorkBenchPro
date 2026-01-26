use std::fs;
use std::path::PathBuf;
use std::process::Command;

use anyhow::Result;

use crate::benchmarks::{Benchmark, Category, ProgressCallback};
use crate::core::{CommandExt, Timer};
use crate::models::{TestDetails, TestResult};

/// Archive operations benchmark - tests real archive compress/extract performance
pub struct ArchiveOpsBenchmark {
    test_dir: PathBuf,
}

impl ArchiveOpsBenchmark {
    pub fn new() -> Self {
        Self {
            test_dir: std::env::temp_dir().join("workbench_pro_archive_test"),
        }
    }

    fn is_tar_available() -> bool {
        Command::new("tar")
            .arg("--version")
            .hidden()
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    fn setup_test_files(&self, progress: &dyn ProgressCallback) -> Result<PathBuf> {
        let source_dir = self.test_dir.join("source");

        // Clean up any existing test directory
        let _ = fs::remove_dir_all(&self.test_dir);
        fs::create_dir_all(&source_dir)?;

        progress.update(0.05, "Creating test files...");

        // Create a realistic file structure (simulating a project)
        let num_dirs = 30;
        let files_per_dir = 50;

        for d in 0..num_dirs {
            if progress.is_cancelled() {
                return Err(anyhow::anyhow!("Cancelled"));
            }

            let dir_path = source_dir.join(format!("module_{:02}", d));
            fs::create_dir_all(&dir_path)?;

            for f in 0..files_per_dir {
                let file_path = dir_path.join(format!("file_{:03}.txt", f));

                // Create varied content sizes
                let size = match f % 5 {
                    0 => 1024,      // 1KB
                    1 => 4096,      // 4KB
                    2 => 16384,     // 16KB
                    3 => 65536,     // 64KB
                    _ => 2048,      // 2KB
                };

                // Create realistic text content
                let content: String = (0..size)
                    .map(|i| {
                        let c = (((d * 17 + f * 13 + i) % 62) as u8);
                        if c < 26 {
                            (b'a' + c) as char
                        } else if c < 52 {
                            (b'A' + c - 26) as char
                        } else if c < 62 {
                            (b'0' + c - 52) as char
                        } else {
                            ' '
                        }
                    })
                    .collect();

                fs::write(&file_path, content)?;
            }

            if d % 5 == 0 {
                progress.update(
                    0.05 + (d as f32 / num_dirs as f32) * 0.15,
                    &format!("Creating files... {}/{} dirs", d, num_dirs),
                );
            }
        }

        Ok(source_dir)
    }

    fn cleanup(&self) {
        let _ = fs::remove_dir_all(&self.test_dir);
    }
}

impl Default for ArchiveOpsBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl Benchmark for ArchiveOpsBenchmark {
    fn id(&self) -> &'static str {
        "archive_ops"
    }

    fn name(&self) -> &'static str {
        "Archive Operations"
    }

    fn description(&self) -> &'static str {
        "Compress and extract 1500 files (~50MB) using tar"
    }

    fn category(&self) -> Category {
        Category::BuildPerformance
    }

    fn estimated_duration_secs(&self) -> u32 {
        45
    }

    fn run(&self, progress: &dyn ProgressCallback) -> Result<TestResult> {
        // Check if tar is available
        if !Self::is_tar_available() {
            return Err(anyhow::anyhow!("tar is not installed or not in PATH"));
        }

        // Setup test files
        let source_dir = self.setup_test_files(progress)?;

        let archive_path = self.test_dir.join("archive.tar.gz");
        let extract_dir = self.test_dir.join("extracted");

        progress.update(0.2, "Running archive benchmark...");

        let mut compress_times: Vec<f64> = Vec::new();
        let mut extract_times: Vec<f64> = Vec::new();

        for i in 0..5 {
            if progress.is_cancelled() {
                self.cleanup();
                return Err(anyhow::anyhow!("Cancelled"));
            }

            // Clean up previous
            let _ = fs::remove_file(&archive_path);
            let _ = fs::remove_dir_all(&extract_dir);

            // Compress
            let timer = Timer::new();
            let output = Command::new("tar")
                .args(["-czf", archive_path.to_str().unwrap(), "-C", source_dir.to_str().unwrap(), "."])
                .hidden()
                .output()?;

            if !output.status.success() {
                self.cleanup();
                return Err(anyhow::anyhow!("tar compress failed"));
            }
            compress_times.push(timer.elapsed_secs());

            progress.update(
                0.2 + (i as f32 / 5.0) * 0.3,
                &format!("Compress {}/5...", i + 1),
            );

            // Extract
            fs::create_dir_all(&extract_dir)?;

            let timer = Timer::new();
            let output = Command::new("tar")
                .args(["-xzf", archive_path.to_str().unwrap(), "-C", extract_dir.to_str().unwrap()])
                .hidden()
                .output()?;

            if !output.status.success() {
                self.cleanup();
                return Err(anyhow::anyhow!("tar extract failed"));
            }
            extract_times.push(timer.elapsed_secs());

            progress.update(
                0.5 + (i as f32 / 5.0) * 0.35,
                &format!("Extract {}/5...", i + 1),
            );
        }

        // Cleanup
        progress.update(0.9, "Cleaning up...");
        self.cleanup();

        // Calculate statistics
        let avg_compress = compress_times.iter().sum::<f64>() / compress_times.len() as f64;
        let avg_extract = extract_times.iter().sum::<f64>() / extract_times.len() as f64;
        let avg_total = avg_compress + avg_extract;

        let all_times: Vec<f64> = [compress_times, extract_times].concat();
        let min = all_times.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = all_times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        progress.update(1.0, "Complete");

        Ok(TestResult {
            test_id: self.id().to_string(),
            name: self.name().to_string(),
            description: format!(
                "{} (compress: {:.2}s, extract: {:.2}s)",
                self.description(),
                avg_compress,
                avg_extract
            ),
            value: avg_total,
            unit: "s".to_string(),
            details: TestDetails {
                iterations: 10,
                duration_secs: all_times.iter().sum(),
                min,
                max,
                mean: avg_total,
                median: avg_total,
                std_dev: 0.0,
                percentiles: None,
            },
        })
    }
}
