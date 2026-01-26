use anyhow::Result;

use crate::models::TestResult;

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
    fn run(&self, progress: &dyn ProgressCallback) -> Result<TestResult>;
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
