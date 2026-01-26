use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::system_info::SystemInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkRun {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub machine_name: String,
    pub notes: Option<String>,
    pub tags: Vec<String>,
    pub system_info: SystemInfo,
    pub results: CategoryResults,
    /// Optional remote ID for online comparison service
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_id: Option<String>,
    /// Timestamp when results were uploaded to remote service
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uploaded_at: Option<DateTime<Utc>>,
}

impl BenchmarkRun {
    pub fn new(machine_name: String, system_info: SystemInfo) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            machine_name,
            notes: None,
            tags: Vec::new(),
            system_info,
            results: CategoryResults::default(),
            remote_id: None,
            uploaded_at: None,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CategoryResults {
    pub project_operations: Vec<TestResult>,
    pub build_performance: Vec<TestResult>,
    pub responsiveness: Vec<TestResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub test_id: String,
    pub name: String,
    pub description: String,
    pub value: f64,
    pub unit: String,
    pub details: TestDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestDetails {
    pub iterations: u32,
    pub duration_secs: f64,
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub median: f64,
    pub std_dev: f64,
    pub percentiles: Option<Percentiles>,
}

impl Default for TestDetails {
    fn default() -> Self {
        Self {
            iterations: 0,
            duration_secs: 0.0,
            min: 0.0,
            max: 0.0,
            mean: 0.0,
            median: 0.0,
            std_dev: 0.0,
            percentiles: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Percentiles {
    pub p50: f64,
    pub p75: f64,
    pub p90: f64,
    pub p95: f64,
    pub p99: f64,
    pub p999: f64,
}

impl Percentiles {
    pub fn from_sorted_values(sorted_values: &[f64]) -> Self {
        let len = sorted_values.len();
        if len == 0 {
            return Self {
                p50: 0.0,
                p75: 0.0,
                p90: 0.0,
                p95: 0.0,
                p99: 0.0,
                p999: 0.0,
            };
        }

        let percentile = |p: f64| -> f64 {
            let idx = ((p / 100.0) * (len - 1) as f64).round() as usize;
            sorted_values[idx.min(len - 1)]
        };

        Self {
            p50: percentile(50.0),
            p75: percentile(75.0),
            p90: percentile(90.0),
            p95: percentile(95.0),
            p99: percentile(99.0),
            p999: percentile(99.9),
        }
    }
}

