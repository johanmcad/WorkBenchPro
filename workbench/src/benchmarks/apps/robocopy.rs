use std::fs;
use std::path::PathBuf;
use std::process::Command;

use anyhow::Result;

use crate::benchmarks::{Benchmark, Category, ProgressCallback};
use crate::core::Timer;
use crate::models::{TestDetails, TestResult};

/// Robocopy benchmark - tests Windows robust file copy performance
/// Robocopy is the recommended tool for file operations on Windows
pub struct RobocopyBenchmark {
    test_dir: PathBuf,
}

impl RobocopyBenchmark {
    pub fn new() -> Self {
        Self {
            test_dir: std::env::temp_dir().join("workbench_robocopy_test"),
        }
    }

    fn is_robocopy_available() -> bool {
        // Just check if robocopy can be executed - any output means it exists
        // robocopy /? returns exit code 16 but still produces help output
        Command::new("robocopy")
            .arg("/?")
            .output()
            .map(|o| !o.stdout.is_empty() || !o.stderr.is_empty())
            .unwrap_or(false)
    }

    fn setup_source_files(&self, progress: &dyn ProgressCallback) -> Result<PathBuf> {
        let source_dir = self.test_dir.join("source");

        // Clean up any existing test directory
        let _ = fs::remove_dir_all(&self.test_dir);
        fs::create_dir_all(&source_dir)?;

        progress.update(0.05, "Creating source files...");

        // Create a realistic directory structure
        let num_dirs = 20;
        let files_per_dir = 50;

        for d in 0..num_dirs {
            if progress.is_cancelled() {
                return Err(anyhow::anyhow!("Cancelled"));
            }

            let dir_path = source_dir.join(format!("folder_{:02}", d));
            fs::create_dir_all(&dir_path)?;

            // Create subdirectories
            let sub_dir = dir_path.join("subdir");
            fs::create_dir_all(&sub_dir)?;

            for f in 0..files_per_dir {
                // Vary file sizes
                let size = match f % 5 {
                    0 => 1024,       // 1KB small files
                    1 => 4096,       // 4KB
                    2 => 16384,      // 16KB
                    3 => 65536,      // 64KB
                    _ => 8192,       // 8KB
                };

                // Create content with some structure
                let content: Vec<u8> = (0..size)
                    .map(|i| ((d * 17 + f * 13 + i) % 256) as u8)
                    .collect();

                let file_path = dir_path.join(format!("file_{:03}.dat", f));
                fs::write(&file_path, &content)?;

                // Some files in subdirectory
                if f < 10 {
                    let sub_file = sub_dir.join(format!("subfile_{:03}.dat", f));
                    fs::write(&sub_file, &content)?;
                }
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

impl Default for RobocopyBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl Benchmark for RobocopyBenchmark {
    fn id(&self) -> &'static str {
        "robocopy"
    }

    fn name(&self) -> &'static str {
        "Robocopy File Copy"
    }

    fn description(&self) -> &'static str {
        "Copy 1200+ files across directories using robocopy"
    }

    fn category(&self) -> Category {
        Category::ProjectOperations
    }

    fn estimated_duration_secs(&self) -> u32 {
        45
    }

    fn run(&self, progress: &dyn ProgressCallback) -> Result<TestResult> {
        // Check if robocopy is available
        if !Self::is_robocopy_available() {
            return Err(anyhow::anyhow!("Robocopy is not available (Windows only)"));
        }

        // Setup source files
        let source_dir = self.setup_source_files(progress)?;

        progress.update(0.2, "Running robocopy benchmarks...");

        let mut copy_times: Vec<f64> = Vec::new();
        let mut mirror_times: Vec<f64> = Vec::new();

        for i in 0..5 {
            if progress.is_cancelled() {
                self.cleanup();
                return Err(anyhow::anyhow!("Cancelled"));
            }

            let dest_copy = self.test_dir.join(format!("dest_copy_{}", i));
            let dest_mirror = self.test_dir.join(format!("dest_mirror_{}", i));

            // Clean destinations
            let _ = fs::remove_dir_all(&dest_copy);
            let _ = fs::remove_dir_all(&dest_mirror);

            // Test 1: Standard copy with /E (recursive)
            let timer = Timer::new();
            let output = Command::new("robocopy")
                .args([
                    source_dir.to_str().unwrap(),
                    dest_copy.to_str().unwrap(),
                    "/E",      // Copy subdirectories including empty ones
                    "/NP",     // No progress
                    "/NFL",    // No file list
                    "/NDL",    // No directory list
                    "/NJH",    // No job header
                    "/NJS",    // No job summary
                    "/MT:4",   // Multi-threaded (4 threads)
                ])
                .output()?;

            let copy_time = timer.elapsed_secs();

            // Robocopy exit codes: 0-7 are success, 8+ are errors
            if output.status.code().map(|c| c >= 8).unwrap_or(true) {
                self.cleanup();
                return Err(anyhow::anyhow!("Robocopy copy failed"));
            }
            copy_times.push(copy_time);

            progress.update(
                0.2 + (i as f32 / 5.0) * 0.3,
                &format!("Copy test {}/5...", i + 1),
            );

            // Test 2: Mirror copy with /MIR
            let timer = Timer::new();
            let output = Command::new("robocopy")
                .args([
                    source_dir.to_str().unwrap(),
                    dest_mirror.to_str().unwrap(),
                    "/MIR",    // Mirror directory tree
                    "/NP",
                    "/NFL",
                    "/NDL",
                    "/NJH",
                    "/NJS",
                    "/MT:4",
                ])
                .output()?;

            let mirror_time = timer.elapsed_secs();

            if output.status.code().map(|c| c >= 8).unwrap_or(true) {
                self.cleanup();
                return Err(anyhow::anyhow!("Robocopy mirror failed"));
            }
            mirror_times.push(mirror_time);

            progress.update(
                0.5 + (i as f32 / 5.0) * 0.4,
                &format!("Mirror test {}/5...", i + 1),
            );
        }

        // Cleanup
        progress.update(0.95, "Cleaning up...");
        self.cleanup();

        // Calculate statistics
        let avg_copy = copy_times.iter().sum::<f64>() / copy_times.len() as f64;
        let avg_mirror = mirror_times.iter().sum::<f64>() / mirror_times.len() as f64;
        let avg_total = (avg_copy + avg_mirror) / 2.0;

        let all_times: Vec<f64> = [copy_times, mirror_times].concat();
        let min = all_times.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = all_times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        progress.update(1.0, "Complete");

        Ok(TestResult {
            test_id: self.id().to_string(),
            name: self.name().to_string(),
            description: format!(
                "{} (copy: {:.2}s, mirror: {:.2}s)",
                self.description(),
                avg_copy,
                avg_mirror
            ),
            value: avg_total,
            unit: "s".to_string(),
            details: TestDetails {
                iterations: all_times.len() as u32,
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
