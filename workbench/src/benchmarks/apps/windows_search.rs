use std::fs;
use std::path::PathBuf;
use std::process::Command;

use anyhow::Result;

use crate::benchmarks::{Benchmark, Category, ProgressCallback};
use crate::core::Timer;
use crate::models::{TestDetails, TestResult};

/// Windows Search / File indexing benchmark
/// Tests file search performance using native OS search tools
pub struct WindowsSearchBenchmark {
    test_dir: PathBuf,
}

impl WindowsSearchBenchmark {
    pub fn new() -> Self {
        Self {
            test_dir: std::env::temp_dir().join("workbench_search_test"),
        }
    }

    fn setup_searchable_content(&self, progress: &dyn ProgressCallback) -> Result<()> {
        // Clean up any existing test directory
        let _ = fs::remove_dir_all(&self.test_dir);
        fs::create_dir_all(&self.test_dir)?;

        progress.update(0.05, "Creating searchable content...");

        // Create a directory structure with varied content
        let keywords = [
            "function", "class", "import", "export", "async", "await",
            "interface", "struct", "enum", "const", "let", "var",
            "return", "if", "else", "for", "while", "match",
            "error", "result", "option", "some", "none", "ok",
        ];

        let extensions = [".rs", ".js", ".ts", ".py", ".go", ".java", ".cpp", ".h"];

        let num_dirs = 20;
        let files_per_dir = 50;

        for d in 0..num_dirs {
            if progress.is_cancelled() {
                return Err(anyhow::anyhow!("Cancelled"));
            }

            let dir_path = self.test_dir.join(format!("src/module_{:02}", d));
            fs::create_dir_all(&dir_path)?;

            // Add nested subdirectories
            let sub_dir = dir_path.join("sub");
            fs::create_dir_all(&sub_dir)?;

            for f in 0..files_per_dir {
                let ext = extensions[(d + f) % extensions.len()];
                let file_path = if f < files_per_dir / 2 {
                    dir_path.join(format!("file_{:03}{}", f, ext))
                } else {
                    sub_dir.join(format!("nested_{:03}{}", f, ext))
                };

                // Generate content with searchable keywords
                let mut content = String::new();
                content.push_str(&format!("// File {} in module {}\n", f, d));

                for (k, keyword) in keywords.iter().enumerate() {
                    if (d + f + k) % 3 == 0 {
                        content.push_str(&format!(
                            "pub {} item_{}_{} {{ /* implementation */ }}\n",
                            keyword, d, f
                        ));
                    }
                }

                // Add some unique identifiable strings
                content.push_str(&format!("\n// UNIQUE_MARKER_{}_{}\n", d, f));

                // Add lorem-ipsum style filler
                for _ in 0..(f % 20) {
                    content.push_str("Lorem ipsum dolor sit amet, consectetur adipiscing elit. ");
                }

                fs::write(&file_path, content)?;
            }

            if d % 5 == 0 {
                progress.update(
                    0.05 + (d as f32 / num_dirs as f32) * 0.15,
                    &format!("Creating content... {}/{} dirs", d, num_dirs),
                );
            }
        }

        Ok(())
    }

    fn cleanup(&self) {
        let _ = fs::remove_dir_all(&self.test_dir);
    }
}

impl Default for WindowsSearchBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl Benchmark for WindowsSearchBenchmark {
    fn id(&self) -> &'static str {
        "file_search"
    }

    fn name(&self) -> &'static str {
        "File Search"
    }

    fn description(&self) -> &'static str {
        "Search through 1000+ files using OS native tools"
    }

    fn category(&self) -> Category {
        Category::ProjectOperations
    }

    fn estimated_duration_secs(&self) -> u32 {
        45
    }

    fn run(&self, progress: &dyn ProgressCallback) -> Result<TestResult> {
        // Setup searchable content
        self.setup_searchable_content(progress)?;

        progress.update(0.2, "Running search benchmarks...");

        let is_windows = cfg!(target_os = "windows");
        let mut filename_times: Vec<f64> = Vec::new();
        let mut content_times: Vec<f64> = Vec::new();
        let mut recursive_times: Vec<f64> = Vec::new();

        let search_patterns = ["function", "class", "UNIQUE_MARKER", "error", "result"];

        // Test 1: Filename search
        progress.update(0.25, "Testing filename search...");

        for i in 0..10 {
            if progress.is_cancelled() {
                self.cleanup();
                return Err(anyhow::anyhow!("Cancelled"));
            }

            let pattern = format!("file_{:03}", i * 5);

            let timer = Timer::new();
            if is_windows {
                // Windows: use dir /s /b
                let _ = Command::new("cmd")
                    .args(["/c", "dir", "/s", "/b"])
                    .current_dir(&self.test_dir)
                    .arg(&format!("*{}*", pattern))
                    .output();
            } else {
                // Unix: use find
                let _ = Command::new("find")
                    .arg(&self.test_dir)
                    .args(["-name", &format!("*{}*", pattern)])
                    .output();
            }
            filename_times.push(timer.elapsed_secs() * 1000.0);
        }

        // Test 2: Content search
        progress.update(0.45, "Testing content search...");

        for (i, pattern) in search_patterns.iter().enumerate() {
            if progress.is_cancelled() {
                self.cleanup();
                return Err(anyhow::anyhow!("Cancelled"));
            }

            progress.update(
                0.45 + (i as f32 / search_patterns.len() as f32) * 0.25,
                &format!("Content search: {}...", pattern),
            );

            let timer = Timer::new();
            if is_windows {
                // Windows: use findstr /s /i
                let _ = Command::new("findstr")
                    .args(["/s", "/i", pattern])
                    .arg(self.test_dir.join("*").to_str().unwrap())
                    .output();
            } else {
                // Unix: use grep -r
                let _ = Command::new("grep")
                    .args(["-r", "-l", pattern])
                    .arg(&self.test_dir)
                    .output();
            }
            content_times.push(timer.elapsed_secs() * 1000.0);
        }

        // Test 3: Recursive directory listing
        progress.update(0.7, "Testing recursive listing...");

        for _ in 0..10 {
            if progress.is_cancelled() {
                self.cleanup();
                return Err(anyhow::anyhow!("Cancelled"));
            }

            let timer = Timer::new();
            if is_windows {
                let _ = Command::new("cmd")
                    .args(["/c", "dir", "/s", "/b"])
                    .current_dir(&self.test_dir)
                    .output();
            } else {
                let _ = Command::new("find")
                    .arg(&self.test_dir)
                    .arg("-type")
                    .arg("f")
                    .output();
            }
            recursive_times.push(timer.elapsed_secs() * 1000.0);
        }

        // Cleanup
        progress.update(0.95, "Cleaning up...");
        self.cleanup();

        // Calculate statistics
        let avg_filename = filename_times.iter().sum::<f64>() / filename_times.len() as f64;
        let avg_content = content_times.iter().sum::<f64>() / content_times.len() as f64;
        let avg_recursive = recursive_times.iter().sum::<f64>() / recursive_times.len() as f64;

        let avg_combined = (avg_filename + avg_content + avg_recursive) / 3.0;

        let all_times: Vec<f64> = [filename_times, content_times, recursive_times].concat();
        let min = all_times.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = all_times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        progress.update(1.0, "Complete");

        Ok(TestResult {
            test_id: self.id().to_string(),
            name: self.name().to_string(),
            description: format!(
                "{} (name: {:.0}ms, content: {:.0}ms, list: {:.0}ms)",
                self.description(),
                avg_filename,
                avg_content,
                avg_recursive
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
