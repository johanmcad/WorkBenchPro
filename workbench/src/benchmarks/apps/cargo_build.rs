use std::fs;
use std::path::PathBuf;
use std::process::Command;

use anyhow::Result;

use crate::benchmarks::{Benchmark, Category, ProgressCallback};
use crate::core::Timer;
use crate::models::{TestDetails, TestResult};

/// Cargo build benchmark - tests real Rust compilation performance
pub struct CargoBuildBenchmark {
    test_dir: PathBuf,
}

impl CargoBuildBenchmark {
    pub fn new() -> Self {
        Self {
            test_dir: std::env::temp_dir().join("workbench_cargo_test"),
        }
    }

    fn is_cargo_available() -> bool {
        Command::new("cargo")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    fn create_test_project(&self, progress: &dyn ProgressCallback) -> Result<PathBuf> {
        let project_dir = self.test_dir.join("bench_project");

        // Clean up any existing test directory
        let _ = fs::remove_dir_all(&self.test_dir);
        fs::create_dir_all(&project_dir)?;

        progress.update(0.05, "Creating Cargo project...");

        // Create Cargo.toml
        let cargo_toml = r#"[package]
name = "bench_project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
"#;
        fs::write(project_dir.join("Cargo.toml"), cargo_toml)?;

        // Create src directory
        fs::create_dir_all(project_dir.join("src"))?;

        progress.update(0.1, "Generating source files...");

        // Generate main.rs with imports
        let main_rs = r#"mod lib;
use lib::*;

fn main() {
    let data = generate_data(100);
    println!("Generated {} items", data.len());

    let result = process_data(&data);
    println!("Processed result: {}", result);
}
"#;
        fs::write(project_dir.join("src/main.rs"), main_rs)?;

        // Generate lib.rs with substantial code
        let mut lib_content = String::from(
            r#"use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataItem {
    pub id: u64,
    pub name: String,
    pub values: Vec<f64>,
}

pub fn generate_data(count: usize) -> Vec<DataItem> {
    (0..count)
        .map(|i| DataItem {
            id: i as u64,
            name: format!("Item_{}", i),
            values: (0..10).map(|j| (i * j) as f64).collect(),
        })
        .collect()
}

pub fn process_data(items: &[DataItem]) -> f64 {
    items
        .iter()
        .flat_map(|item| item.values.iter())
        .sum()
}

"#,
        );

        // Add more functions to increase compilation work
        for i in 0..50 {
            lib_content.push_str(&format!(
                r#"
pub fn compute_{}(x: f64, y: f64) -> f64 {{
    let mut result = x * y;
    for i in 0..{} {{
        result = (result + i as f64).sin().abs();
    }}
    result
}}

pub fn transform_{}(items: &[DataItem]) -> Vec<DataItem> {{
    items
        .iter()
        .map(|item| DataItem {{
            id: item.id + {},
            name: format!("Transformed_{{}}", item.name),
            values: item.values.iter().map(|v| v * {}.0).collect(),
        }})
        .collect()
}}
"#,
                i, i + 10, i, i, i + 1
            ));
        }

        fs::write(project_dir.join("src/lib.rs"), lib_content)?;

        Ok(project_dir)
    }

    fn cleanup(&self) {
        let _ = fs::remove_dir_all(&self.test_dir);
    }
}

impl Default for CargoBuildBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl Benchmark for CargoBuildBenchmark {
    fn id(&self) -> &'static str {
        "cargo_build"
    }

    fn name(&self) -> &'static str {
        "Cargo Build"
    }

    fn description(&self) -> &'static str {
        "Compile a Rust project with dependencies"
    }

    fn category(&self) -> Category {
        Category::BuildPerformance
    }

    fn estimated_duration_secs(&self) -> u32 {
        120
    }

    fn run(&self, progress: &dyn ProgressCallback) -> Result<TestResult> {
        // Check if cargo is available
        if !Self::is_cargo_available() {
            return Err(anyhow::anyhow!("Cargo is not installed or not in PATH"));
        }

        // Create test project
        let project_dir = self.create_test_project(progress)?;

        progress.update(0.15, "Running initial build (with deps)...");

        // First build - includes dependency download and compilation
        let timer_full = Timer::new();
        let output = Command::new("cargo")
            .args(["build", "--release"])
            .current_dir(&project_dir)
            .output()?;

        let full_build_time = timer_full.elapsed_secs();

        if !output.status.success() {
            self.cleanup();
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Cargo build failed: {}", stderr));
        }

        progress.update(0.5, "Running incremental builds...");

        // Incremental builds - modify source and rebuild
        let mut incremental_times: Vec<f64> = Vec::new();

        for i in 0..5 {
            if progress.is_cancelled() {
                self.cleanup();
                return Err(anyhow::anyhow!("Cancelled"));
            }

            // Modify lib.rs slightly
            let lib_path = project_dir.join("src/lib.rs");
            let mut content = fs::read_to_string(&lib_path)?;
            content.push_str(&format!("\n// Modification {}\n", i));
            fs::write(&lib_path, content)?;

            let timer = Timer::new();
            Command::new("cargo")
                .args(["build", "--release"])
                .current_dir(&project_dir)
                .output()?;
            incremental_times.push(timer.elapsed_secs());

            progress.update(
                0.5 + (i as f32 / 5.0) * 0.4,
                &format!("Incremental build {}/5...", i + 1),
            );
        }

        // Cleanup
        progress.update(0.95, "Cleaning up...");
        self.cleanup();

        // Calculate statistics
        let avg_incremental = incremental_times.iter().sum::<f64>() / incremental_times.len() as f64;

        // Score based on incremental build time
        // <2s = 600, <5s = 500, <10s = 400, <20s = 300, <30s = 200, >30s = 100
        let score = if avg_incremental < 2.0 {
            600
        } else if avg_incremental < 5.0 {
            500
        } else if avg_incremental < 10.0 {
            400
        } else if avg_incremental < 20.0 {
            300
        } else if avg_incremental < 30.0 {
            200
        } else {
            100
        };

        let min = incremental_times.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = incremental_times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        progress.update(1.0, "Complete");

        Ok(TestResult {
            test_id: self.id().to_string(),
            name: self.name().to_string(),
            description: format!(
                "{} (full: {:.1}s, incremental avg: {:.1}s)",
                self.description(),
                full_build_time,
                avg_incremental
            ),
            value: avg_incremental,
            unit: "s (incr)".to_string(),
            score,
            max_score: 600,
            details: TestDetails {
                iterations: 6, // 1 full + 5 incremental
                duration_secs: full_build_time + incremental_times.iter().sum::<f64>(),
                min,
                max,
                mean: avg_incremental,
                median: incremental_times[2],
                std_dev: 0.0,
                percentiles: None,
            },
        })
    }
}
