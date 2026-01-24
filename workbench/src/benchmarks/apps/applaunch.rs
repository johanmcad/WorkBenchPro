use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

use anyhow::Result;

use crate::benchmarks::{Benchmark, Category, ProgressCallback};
use crate::core::Timer;
use crate::models::{TestDetails, TestResult};

/// Windows Application Launch benchmark
/// Tests startup time for built-in Windows applications
pub struct AppLaunchBenchmark {
    test_dir: PathBuf,
}

impl AppLaunchBenchmark {
    pub fn new() -> Self {
        Self {
            test_dir: std::env::temp_dir().join("workbench_applaunch_test"),
        }
    }

    fn cleanup(&self) {
        let _ = fs::remove_dir_all(&self.test_dir);
    }

    /// Check if we're on Windows
    fn is_windows() -> bool {
        cfg!(target_os = "windows")
    }

    /// Get list of apps to test based on platform
    fn get_test_apps() -> Vec<(&'static str, Vec<&'static str>)> {
        if cfg!(target_os = "windows") {
            vec![
                ("notepad", vec!["notepad.exe"]),
                ("wordpad", vec!["write.exe"]),  // WordPad executable name
                ("calc", vec!["calc.exe"]),
                ("mspaint", vec!["mspaint.exe"]),
                ("cmd", vec!["cmd.exe", "/c", "echo", "test"]),
            ]
        } else {
            // Linux/Unix equivalents
            vec![
                ("nano", vec!["nano", "--version"]),
                ("vi", vec!["vi", "--version"]),
                ("bc", vec!["bc", "--version"]),
                ("sh", vec!["sh", "-c", "echo test"]),
            ]
        }
    }
}

impl Default for AppLaunchBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl Benchmark for AppLaunchBenchmark {
    fn id(&self) -> &'static str {
        "applaunch"
    }

    fn name(&self) -> &'static str {
        "Application Launch"
    }

    fn description(&self) -> &'static str {
        "Launch built-in applications (Notepad, WordPad, Calculator, etc.)"
    }

    fn category(&self) -> Category {
        Category::Responsiveness
    }

    fn estimated_duration_secs(&self) -> u32 {
        45
    }

    fn run(&self, progress: &dyn ProgressCallback) -> Result<TestResult> {
        // Setup
        let _ = fs::remove_dir_all(&self.test_dir);
        fs::create_dir_all(&self.test_dir)?;

        progress.update(0.05, "Preparing application launch benchmark...");

        // Create a test file for editors to open
        let test_file = self.test_dir.join("test.txt");
        fs::write(&test_file, "WorkBench test file\nLine 2\nLine 3\n")?;

        let apps = Self::get_test_apps();
        let mut all_times: Vec<f64> = Vec::new();
        let mut app_results: Vec<(String, f64)> = Vec::new();

        let is_windows = Self::is_windows();

        for (app_idx, (app_name, args)) in apps.iter().enumerate() {
            if progress.is_cancelled() {
                self.cleanup();
                return Err(anyhow::anyhow!("Cancelled"));
            }

            progress.update(
                0.1 + (app_idx as f32 / apps.len() as f32) * 0.85,
                &format!("Testing {}...", app_name),
            );

            let mut times: Vec<f64> = Vec::new();

            // Run multiple iterations
            for _ in 0..5 {
                if progress.is_cancelled() {
                    self.cleanup();
                    return Err(anyhow::anyhow!("Cancelled"));
                }

                let timer = Timer::new();

                if is_windows {
                    // On Windows, launch app and immediately close it
                    // For GUI apps, we use start /wait with timeout
                    let cmd_name = args[0];

                    // Special handling for different apps
                    match *app_name {
                        "notepad" | "wordpad" | "mspaint" => {
                            // Launch and kill after a brief moment
                            // Use taskkill to terminate the process
                            let mut child = Command::new(cmd_name)
                                .arg(test_file.to_str().unwrap_or(""))
                                .spawn();

                            if let Ok(ref mut c) = child {
                                // Wait a moment for startup
                                std::thread::sleep(Duration::from_millis(100));
                                let _ = c.kill();
                                let _ = c.wait();
                            }
                        }
                        "calc" => {
                            // Calculator - launch and kill
                            let mut child = Command::new(cmd_name).spawn();
                            if let Ok(ref mut c) = child {
                                std::thread::sleep(Duration::from_millis(100));
                                let _ = c.kill();
                                let _ = c.wait();
                            }
                        }
                        "cmd" => {
                            // Command prompt with echo - runs and exits
                            let _ = Command::new(args[0])
                                .args(&args[1..])
                                .output();
                        }
                        _ => {
                            let _ = Command::new(args[0])
                                .args(&args[1..])
                                .output();
                        }
                    }
                } else {
                    // On Linux, run the command (most will just print version and exit)
                    let _ = Command::new(args[0])
                        .args(&args[1..])
                        .output();
                }

                let elapsed = timer.elapsed_secs() * 1000.0;
                times.push(elapsed);
            }

            let avg = times.iter().sum::<f64>() / times.len() as f64;
            app_results.push((app_name.to_string(), avg));
            all_times.extend(times);
        }

        // Cleanup
        progress.update(0.95, "Cleaning up...");
        self.cleanup();

        // Calculate statistics
        let total_avg = app_results.iter().map(|(_, t)| t).sum::<f64>() / app_results.len() as f64;

        // Score: faster is better
        // <50ms = 350, <100ms = 300, <200ms = 250, <500ms = 200, >500ms = 100
        let score = if total_avg < 50.0 {
            350
        } else if total_avg < 100.0 {
            300
        } else if total_avg < 200.0 {
            250
        } else if total_avg < 500.0 {
            200
        } else {
            100
        };

        let min = all_times.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = all_times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        // Build description with app times
        let app_summary: Vec<String> = app_results
            .iter()
            .take(3)
            .map(|(name, time)| format!("{}: {:.0}ms", name, time))
            .collect();

        progress.update(1.0, "Complete");

        Ok(TestResult {
            test_id: self.id().to_string(),
            name: self.name().to_string(),
            description: format!(
                "{} ({})",
                self.description(),
                app_summary.join(", ")
            ),
            value: total_avg,
            unit: "ms".to_string(),
            score,
            max_score: 350,
            details: TestDetails {
                iterations: all_times.len() as u32,
                duration_secs: all_times.iter().sum::<f64>() / 1000.0,
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
