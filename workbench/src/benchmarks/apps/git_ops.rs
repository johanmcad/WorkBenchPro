use std::fs;
use std::path::PathBuf;
use std::process::Command;

use anyhow::Result;

use crate::benchmarks::{Benchmark, Category, ProgressCallback};
use crate::core::Timer;
use crate::models::{TestDetails, TestResult};

/// Git operations benchmark - tests real git performance
/// Creates a repo with many files and measures clone, status, diff operations
pub struct GitOperationsBenchmark {
    test_dir: PathBuf,
}

impl GitOperationsBenchmark {
    pub fn new() -> Self {
        Self {
            test_dir: std::env::temp_dir().join("workbench_git_test"),
        }
    }

    fn is_git_available() -> bool {
        Command::new("git")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    fn setup_repo(&self, progress: &dyn ProgressCallback) -> Result<PathBuf> {
        let repo_dir = self.test_dir.join("test_repo");

        // Clean up any existing test directory
        let _ = fs::remove_dir_all(&self.test_dir);
        fs::create_dir_all(&repo_dir)?;

        progress.update(0.05, "Initializing git repository...");

        // Initialize repo
        Command::new("git")
            .args(["init"])
            .current_dir(&repo_dir)
            .output()?;

        // Configure git
        Command::new("git")
            .args(["config", "user.email", "test@workbench.local"])
            .current_dir(&repo_dir)
            .output()?;
        Command::new("git")
            .args(["config", "user.name", "WorkBench Test"])
            .current_dir(&repo_dir)
            .output()?;

        progress.update(0.1, "Creating test files...");

        // Create directory structure with files
        let num_dirs = 50;
        let files_per_dir = 100;

        for d in 0..num_dirs {
            if progress.is_cancelled() {
                return Err(anyhow::anyhow!("Cancelled"));
            }

            let dir_path = repo_dir.join(format!("src/module_{:03}", d));
            fs::create_dir_all(&dir_path)?;

            for f in 0..files_per_dir {
                let file_path = dir_path.join(format!("file_{:03}.rs", f));
                let content = format!(
                    "// Module {} File {}\n\npub fn function_{}_{} () {{\n    println!(\"Hello from {}.{}\");\n}}\n",
                    d, f, d, f, d, f
                );
                fs::write(&file_path, content)?;
            }

            if d % 10 == 0 {
                progress.update(
                    0.1 + (d as f32 / num_dirs as f32) * 0.2,
                    &format!("Creating files... {}/{} dirs", d, num_dirs),
                );
            }
        }

        progress.update(0.3, "Creating initial commit...");

        // Add and commit
        Command::new("git")
            .args(["add", "."])
            .current_dir(&repo_dir)
            .output()?;

        Command::new("git")
            .args(["commit", "-m", "Initial commit with 5000 files"])
            .current_dir(&repo_dir)
            .output()?;

        Ok(repo_dir)
    }

    fn cleanup(&self) {
        let _ = fs::remove_dir_all(&self.test_dir);
    }
}

impl Default for GitOperationsBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl Benchmark for GitOperationsBenchmark {
    fn id(&self) -> &'static str {
        "git_operations"
    }

    fn name(&self) -> &'static str {
        "Git Operations"
    }

    fn description(&self) -> &'static str {
        "Real git operations: status, diff, log on 5000-file repo"
    }

    fn category(&self) -> Category {
        Category::ProjectOperations
    }

    fn estimated_duration_secs(&self) -> u32 {
        60
    }

    fn run(&self, progress: &dyn ProgressCallback) -> Result<TestResult> {
        // Check if git is available
        if !Self::is_git_available() {
            return Err(anyhow::anyhow!("Git is not installed or not in PATH"));
        }

        // Setup repository
        let repo_dir = self.setup_repo(progress)?;

        progress.update(0.35, "Benchmarking git status...");

        // Benchmark git status
        let mut status_times: Vec<f64> = Vec::new();
        for i in 0..10 {
            let timer = Timer::new();
            Command::new("git")
                .args(["status"])
                .current_dir(&repo_dir)
                .output()?;
            status_times.push(timer.elapsed_secs() * 1000.0);

            if i % 3 == 0 {
                progress.update(0.35 + (i as f32 / 10.0) * 0.15, "Measuring git status...");
            }
        }

        progress.update(0.5, "Modifying files for diff test...");

        // Modify some files for diff testing
        for d in 0..10 {
            let file_path = repo_dir.join(format!("src/module_{:03}/file_050.rs", d));
            let content = format!(
                "// Modified Module {}\n\npub fn modified_function_{} () {{\n    println!(\"Modified!\");\n}}\n",
                d, d
            );
            fs::write(&file_path, content)?;
        }

        progress.update(0.55, "Benchmarking git diff...");

        // Benchmark git diff
        let mut diff_times: Vec<f64> = Vec::new();
        for i in 0..10 {
            let timer = Timer::new();
            Command::new("git")
                .args(["diff"])
                .current_dir(&repo_dir)
                .output()?;
            diff_times.push(timer.elapsed_secs() * 1000.0);

            if i % 3 == 0 {
                progress.update(0.55 + (i as f32 / 10.0) * 0.15, "Measuring git diff...");
            }
        }

        progress.update(0.7, "Benchmarking git log...");

        // Benchmark git log
        let mut log_times: Vec<f64> = Vec::new();
        for i in 0..10 {
            let timer = Timer::new();
            Command::new("git")
                .args(["log", "--oneline", "-100"])
                .current_dir(&repo_dir)
                .output()?;
            log_times.push(timer.elapsed_secs() * 1000.0);

            if i % 3 == 0 {
                progress.update(0.7 + (i as f32 / 10.0) * 0.15, "Measuring git log...");
            }
        }

        progress.update(0.85, "Benchmarking git add...");

        // Benchmark git add
        let mut add_times: Vec<f64> = Vec::new();
        for _ in 0..5 {
            // Reset changes
            Command::new("git")
                .args(["checkout", "."])
                .current_dir(&repo_dir)
                .output()?;

            // Modify files again
            for d in 0..10 {
                let file_path = repo_dir.join(format!("src/module_{:03}/file_050.rs", d));
                fs::write(&file_path, format!("// Change {}", d))?;
            }

            let timer = Timer::new();
            Command::new("git")
                .args(["add", "."])
                .current_dir(&repo_dir)
                .output()?;
            add_times.push(timer.elapsed_secs() * 1000.0);
        }

        // Cleanup
        progress.update(0.95, "Cleaning up...");
        self.cleanup();

        // Calculate combined score
        let avg_status = status_times.iter().sum::<f64>() / status_times.len() as f64;
        let avg_diff = diff_times.iter().sum::<f64>() / diff_times.len() as f64;
        let avg_log = log_times.iter().sum::<f64>() / log_times.len() as f64;
        let avg_add = add_times.iter().sum::<f64>() / add_times.len() as f64;

        // Combined average
        let all_times: Vec<f64> = [status_times, diff_times, log_times, add_times].concat();
        all_times.iter().copied().collect::<Vec<f64>>();

        let avg_combined = (avg_status + avg_diff + avg_log + avg_add) / 4.0;

        // Score: faster is better (500 pts max)
        // <50ms avg = 500, <100ms = 400, <200ms = 300, <500ms = 200, >500ms = 100
        let score = if avg_combined < 50.0 {
            500
        } else if avg_combined < 100.0 {
            400
        } else if avg_combined < 200.0 {
            300
        } else if avg_combined < 500.0 {
            200
        } else {
            100
        };

        let min = all_times.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = all_times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        progress.update(1.0, "Complete");

        Ok(TestResult {
            test_id: self.id().to_string(),
            name: self.name().to_string(),
            description: self.description().to_string(),
            value: avg_combined,
            unit: "ms avg".to_string(),
            score,
            max_score: 500,
            details: TestDetails {
                iterations: 35,
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
