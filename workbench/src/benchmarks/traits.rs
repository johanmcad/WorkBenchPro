use std::path::PathBuf;

use anyhow::Result;

use crate::models::TestResult;

/// Configuration passed to benchmarks (uses Quick preset values for fast execution)
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    /// Number of iterations to run
    pub iterations: u32,
    /// Custom test path (None = system temp)
    pub test_path: Option<PathBuf>,

    // Disk settings
    pub disk_file_enum_count: u32,
    pub disk_large_file_mb: u32,
    pub disk_random_read_file_mb: u32,
    pub disk_random_read_count: u32,
    pub disk_metadata_count: u32,
    pub disk_traversal_count: u32,

    // CPU settings
    pub cpu_single_thread_mb: u32,
    pub cpu_multi_thread_chunks: u32,
    pub cpu_mixed_file_count: u32,
    pub cpu_sustained_write_gb: u32,

    // Memory settings
    pub mem_bandwidth_buffer_mb: u32,
    pub mem_latency_buffer_mb: u32,
    pub mem_latency_chase_millions: u32,

    // Latency settings
    pub lat_process_spawn_count: u32,
    pub lat_storage_read_count: u32,
    pub lat_thread_wake_count: u32,

    // App settings
    pub app_csharp_files: u32,
    pub app_csharp_functions: u32,
    pub app_archive_files: u32,
    pub app_compression_files: u32,
    pub app_robocopy_files: u32,
    pub app_defender_files: u32,
}

impl BenchmarkConfig {
    /// Get base test directory
    pub fn test_dir(&self) -> PathBuf {
        self.test_path.clone().unwrap_or_else(std::env::temp_dir)
    }
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            iterations: 3,
            test_path: None,

            // Disk - Quick preset (fast execution)
            disk_file_enum_count: 10_000,
            disk_large_file_mb: 512,
            disk_random_read_file_mb: 256,
            disk_random_read_count: 5_000,
            disk_metadata_count: 2_000,
            disk_traversal_count: 10_000,

            // CPU - Quick preset
            cpu_single_thread_mb: 64,
            cpu_multi_thread_chunks: 500,
            cpu_mixed_file_count: 200,
            cpu_sustained_write_gb: 1,

            // Memory - Quick preset
            mem_bandwidth_buffer_mb: 64,
            mem_latency_buffer_mb: 32,
            mem_latency_chase_millions: 5,

            // Latency - Quick preset
            lat_process_spawn_count: 50,
            lat_storage_read_count: 5_000,
            lat_thread_wake_count: 500,

            // Apps - Quick preset
            app_csharp_files: 3,
            app_csharp_functions: 15,
            app_archive_files: 250,
            app_compression_files: 250,
            app_robocopy_files: 400,
            app_defender_files: 50,
        }
    }
}

/// Category of benchmark tests
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    ProjectOperations,
    BuildPerformance,
    Responsiveness,
}

impl Category {
    pub fn label(&self) -> &'static str {
        match self {
            Category::ProjectOperations => "Project Operations",
            Category::BuildPerformance => "Build Performance",
            Category::Responsiveness => "Responsiveness",
        }
    }

    pub fn max_score(&self) -> u32 {
        2500
    }
}

/// Trait for implementing a benchmark test
pub trait Benchmark: Send + Sync {
    /// Unique identifier for this benchmark
    fn id(&self) -> &'static str;

    /// Human-readable name
    fn name(&self) -> &'static str;

    /// Description of what this benchmark measures
    fn description(&self) -> &'static str;

    /// Which category this benchmark belongs to
    fn category(&self) -> Category;

    /// Estimated duration in seconds
    fn estimated_duration_secs(&self) -> u32;

    /// Whether this is a synthetic benchmark (algorithmic/microbenchmark)
    /// vs a real-world application benchmark.
    /// Default is false (real benchmark).
    fn is_synthetic(&self) -> bool {
        false
    }

    /// Run the benchmark and return results
    fn run(&self, progress: &dyn ProgressCallback, config: &BenchmarkConfig) -> Result<TestResult>;
}

/// Callback for reporting progress during benchmark execution
pub trait ProgressCallback: Send + Sync {
    /// Update progress (0.0 - 1.0) with a message
    fn update(&self, progress: f32, message: &str);

    /// Check if the benchmark should be cancelled
    fn is_cancelled(&self) -> bool;
}

/// No-op progress callback for testing
pub struct NoOpProgress;

impl ProgressCallback for NoOpProgress {
    fn update(&self, _progress: f32, _message: &str) {}
    fn is_cancelled(&self) -> bool {
        false
    }
}
