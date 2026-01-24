use std::fs;
use std::path::PathBuf;

use anyhow::Result;

use crate::benchmarks::{Benchmark, Category, ProgressCallback};
use crate::core::Timer;
use crate::models::{TestDetails, TestResult};

/// Windows Defender impact benchmark - measures the overhead of real-time scanning
/// This tests file creation/modification performance which is affected by antivirus
pub struct DefenderImpactBenchmark {
    test_dir: PathBuf,
}

impl DefenderImpactBenchmark {
    pub fn new() -> Self {
        Self {
            test_dir: std::env::temp_dir().join("workbench_defender_test"),
        }
    }

    fn cleanup(&self) {
        let _ = fs::remove_dir_all(&self.test_dir);
    }
}

impl Default for DefenderImpactBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl Benchmark for DefenderImpactBenchmark {
    fn id(&self) -> &'static str {
        "defender_impact"
    }

    fn name(&self) -> &'static str {
        "Antivirus Impact"
    }

    fn description(&self) -> &'static str {
        "Measure file operation overhead from real-time AV scanning"
    }

    fn category(&self) -> Category {
        Category::ProjectOperations
    }

    fn estimated_duration_secs(&self) -> u32 {
        60
    }

    fn run(&self, progress: &dyn ProgressCallback) -> Result<TestResult> {
        // This benchmark works on any OS but is most relevant on Windows with Defender

        // Clean up any existing test directory
        let _ = fs::remove_dir_all(&self.test_dir);
        fs::create_dir_all(&self.test_dir)?;

        progress.update(0.05, "Preparing benchmark...");

        let mut create_times: Vec<f64> = Vec::new();
        let mut modify_times: Vec<f64> = Vec::new();
        let mut read_times: Vec<f64> = Vec::new();
        let mut delete_times: Vec<f64> = Vec::new();

        let iterations = 5;
        let files_per_iteration = 100;

        for iter in 0..iterations {
            if progress.is_cancelled() {
                self.cleanup();
                return Err(anyhow::anyhow!("Cancelled"));
            }

            let iter_dir = self.test_dir.join(format!("iter_{}", iter));
            fs::create_dir_all(&iter_dir)?;

            // Test 1: File creation with scannable extensions
            progress.update(
                0.1 + (iter as f32 / iterations as f32) * 0.2,
                &format!("File creation test {}/{}...", iter + 1, iterations),
            );

            let timer = Timer::new();
            let files = {
                let mut files = Vec::new();
                let extensions = [".exe", ".dll", ".ps1", ".bat", ".js"];

                for i in 0..files_per_iteration {
                    let ext = extensions[i % extensions.len()];
                    let file_path = iter_dir.join(format!("create_{:04}{}", i, ext));

                    let content: Vec<u8> = if ext == ".exe" || ext == ".dll" {
                        let mut data = vec![0x4D, 0x5A];
                        data.extend(vec![0u8; 512]);
                        data
                    } else {
                        format!("test content for file {}", i).into_bytes()
                    };

                    fs::write(&file_path, content)?;
                    files.push(file_path);
                }
                files
            };
            create_times.push(timer.elapsed_secs() * 1000.0);

            // Test 2: File modification
            progress.update(
                0.3 + (iter as f32 / iterations as f32) * 0.2,
                &format!("File modification test {}/{}...", iter + 1, iterations),
            );

            let timer = Timer::new();
            for file in &files {
                let existing = fs::read(file)?;
                let mut modified = existing;
                modified.extend_from_slice(b"\n// Modified\n");
                fs::write(file, modified)?;
            }
            modify_times.push(timer.elapsed_secs() * 1000.0);

            // Test 3: File reading
            progress.update(
                0.5 + (iter as f32 / iterations as f32) * 0.2,
                &format!("File read test {}/{}...", iter + 1, iterations),
            );

            let timer = Timer::new();
            for file in &files {
                let _ = fs::read(file)?;
            }
            read_times.push(timer.elapsed_secs() * 1000.0);

            // Test 4: File deletion
            progress.update(
                0.7 + (iter as f32 / iterations as f32) * 0.2,
                &format!("File deletion test {}/{}...", iter + 1, iterations),
            );

            let timer = Timer::new();
            for file in &files {
                fs::remove_file(file)?;
            }
            delete_times.push(timer.elapsed_secs() * 1000.0);
        }

        // Cleanup
        progress.update(0.95, "Cleaning up...");
        self.cleanup();

        // Calculate statistics
        let avg_create = create_times.iter().sum::<f64>() / create_times.len() as f64;
        let avg_modify = modify_times.iter().sum::<f64>() / modify_times.len() as f64;
        let avg_read = read_times.iter().sum::<f64>() / read_times.len() as f64;
        let avg_delete = delete_times.iter().sum::<f64>() / delete_times.len() as f64;

        // Calculate overhead per file operation
        let total_ops = files_per_iteration as f64 * 4.0; // create, modify, read, delete
        let total_time = avg_create + avg_modify + avg_read + avg_delete;
        let avg_per_op = total_time / total_ops;

        // Score: lower overhead is better
        // <0.5ms/op = 400, <1ms = 350, <2ms = 300, <5ms = 200, >5ms = 100
        let score = if avg_per_op < 0.5 {
            400
        } else if avg_per_op < 1.0 {
            350
        } else if avg_per_op < 2.0 {
            300
        } else if avg_per_op < 5.0 {
            200
        } else {
            100
        };

        let all_times: Vec<f64> = [create_times, modify_times, read_times, delete_times].concat();
        let min = all_times.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = all_times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        progress.update(1.0, "Complete");

        Ok(TestResult {
            test_id: self.id().to_string(),
            name: self.name().to_string(),
            description: format!(
                "{} (create: {:.0}ms, modify: {:.0}ms, read: {:.0}ms, del: {:.0}ms)",
                self.description(),
                avg_create,
                avg_modify,
                avg_read,
                avg_delete
            ),
            value: avg_per_op,
            unit: "ms/op".to_string(),
            score,
            max_score: 400,
            details: TestDetails {
                iterations: (iterations * 4) as u32,
                duration_secs: all_times.iter().sum::<f64>() / 1000.0,
                min,
                max,
                mean: total_time / 4.0,
                median: total_time / 4.0,
                std_dev: 0.0,
                percentiles: None,
            },
        })
    }
}
