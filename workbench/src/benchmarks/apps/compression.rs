//! Windows built-in compression benchmark using PowerShell Compress-Archive/Expand-Archive

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use anyhow::Result;

use crate::benchmarks::{Benchmark, Category, ProgressCallback};
use crate::core::{CommandExt, Timer};
use crate::models::{TestDetails, TestResult};

/// Windows Compression benchmark - tests compress/decompress using Windows built-in tools
/// Uses PowerShell's Compress-Archive and Expand-Archive cmdlets
pub struct WindowsCompressionBenchmark {
    test_dir: PathBuf,
}

impl WindowsCompressionBenchmark {
    pub fn new() -> Self {
        Self {
            test_dir: std::env::temp_dir().join("workbench_pro_compression_test"),
        }
    }

    fn is_available() -> bool {
        // Check if PowerShell with Compress-Archive is available
        Command::new("powershell")
            .args(["-Command", "Get-Command Compress-Archive -ErrorAction SilentlyContinue"])
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

        progress.update(0.05, "Creating test files for compression...");

        // Create a realistic file structure (simulating a project)
        // 30 directories with ~50 files each = 1500 files
        let num_dirs = 30;
        let files_per_dir = 50;

        for d in 0..num_dirs {
            let dir_path = source_dir.join(format!("folder_{:03}", d));
            fs::create_dir_all(&dir_path)?;

            for f in 0..files_per_dir {
                let file_path = dir_path.join(format!("file_{:04}.txt", f));

                // Create varied content - mix of text and pseudo-code
                let content: String = (0..100)
                    .map(|i| {
                        let c = ((d * 17 + f * 13 + i) % 62) as u8;
                        let ch = match c {
                            0..=25 => (b'a' + c) as char,
                            26..=51 => (b'A' + c - 26) as char,
                            52..=61 => (b'0' + c - 52) as char,
                            _ => ' ',
                        };
                        if i % 20 == 0 { '\n' } else { ch }
                    })
                    .collect();

                fs::write(&file_path, content.repeat(50))?; // ~5KB per file
            }

            if d % 5 == 0 {
                progress.update(
                    0.05 + (d as f32 / num_dirs as f32) * 0.15,
                    &format!("Creating folders... {}/{}", d + 1, num_dirs),
                );
            }
        }

        Ok(source_dir)
    }

    fn cleanup(&self) {
        let _ = fs::remove_dir_all(&self.test_dir);
    }
}

impl Default for WindowsCompressionBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl Benchmark for WindowsCompressionBenchmark {
    fn id(&self) -> &'static str {
        "windows_compression"
    }

    fn name(&self) -> &'static str {
        "Windows Compression"
    }

    fn description(&self) -> &'static str {
        "Compress/extract files using Windows built-in zip (PowerShell)"
    }

    fn category(&self) -> Category {
        Category::BuildPerformance
    }

    fn estimated_duration_secs(&self) -> u32 {
        60
    }

    fn run(&self, progress: &dyn ProgressCallback) -> Result<TestResult> {
        if !Self::is_available() {
            return Err(anyhow::anyhow!(
                "PowerShell Compress-Archive not available (requires Windows PowerShell 5.0+)"
            ));
        }

        let source_dir = self.setup_test_files(progress)?;
        let archive_path = self.test_dir.join("test_archive.zip");
        let extract_dir = self.test_dir.join("extracted");

        let mut compress_times: Vec<f64> = Vec::new();
        let mut extract_times: Vec<f64> = Vec::new();
        let iterations = 5;

        progress.update(0.2, "Running compression benchmark...");

        for i in 0..iterations {
            if progress.is_cancelled() {
                self.cleanup();
                return Err(anyhow::anyhow!("Cancelled"));
            }

            // Clean up from previous iteration
            let _ = fs::remove_file(&archive_path);
            let _ = fs::remove_dir_all(&extract_dir);
            fs::create_dir_all(&extract_dir)?;

            // Compress using PowerShell Compress-Archive
            let timer = Timer::new();
            let compress_cmd = format!(
                "Compress-Archive -Path '{}\\*' -DestinationPath '{}' -CompressionLevel Optimal -Force",
                source_dir.display(),
                archive_path.display()
            );

            let output = Command::new("powershell")
                .args(["-NoProfile", "-Command", &compress_cmd])
                .hidden()
                .output()?;

            if !output.status.success() {
                self.cleanup();
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(anyhow::anyhow!("Compress-Archive failed: {}", stderr));
            }
            compress_times.push(timer.elapsed_secs());

            progress.update(
                0.2 + (i as f32 / iterations as f32) * 0.35,
                &format!("Compression {}/{}...", i + 1, iterations),
            );

            // Extract using PowerShell Expand-Archive
            let timer = Timer::new();
            let extract_cmd = format!(
                "Expand-Archive -Path '{}' -DestinationPath '{}' -Force",
                archive_path.display(),
                extract_dir.display()
            );

            let output = Command::new("powershell")
                .args(["-NoProfile", "-Command", &extract_cmd])
                .hidden()
                .output()?;

            if !output.status.success() {
                self.cleanup();
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(anyhow::anyhow!("Expand-Archive failed: {}", stderr));
            }
            extract_times.push(timer.elapsed_secs());

            progress.update(
                0.55 + (i as f32 / iterations as f32) * 0.35,
                &format!("Extraction {}/{}...", i + 1, iterations),
            );
        }

        // Get archive size for stats
        let archive_size_mb = fs::metadata(&archive_path)
            .map(|m| m.len() as f64 / (1024.0 * 1024.0))
            .unwrap_or(0.0);

        // Cleanup
        progress.update(0.95, "Cleaning up...");
        self.cleanup();

        // Calculate statistics
        let avg_compress = compress_times.iter().sum::<f64>() / compress_times.len() as f64;
        let avg_extract = extract_times.iter().sum::<f64>() / extract_times.len() as f64;
        let total_avg = avg_compress + avg_extract;

        let all_times: Vec<f64> = compress_times
            .iter()
            .chain(extract_times.iter())
            .cloned()
            .collect();

        let min = all_times.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = all_times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        progress.update(1.0, "Complete");

        Ok(TestResult {
            test_id: self.id().to_string(),
            name: self.name().to_string(),
            description: format!(
                "{} (compress: {:.2}s, extract: {:.2}s, archive: {:.1}MB)",
                self.description(),
                avg_compress,
                avg_extract,
                archive_size_mb
            ),
            value: total_avg,
            unit: "sec".to_string(),
            details: TestDetails {
                iterations: (iterations * 2) as u32,
                duration_secs: all_times.iter().sum(),
                min,
                max,
                mean: total_avg,
                median: total_avg,
                std_dev: 0.0,
                percentiles: None,
            },
        })
    }
}
