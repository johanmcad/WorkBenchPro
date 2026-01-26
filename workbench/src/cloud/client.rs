//! HTTP client for Supabase community database

use chrono::{DateTime, Utc};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::models::{BenchmarkRun, StorageType};

// Supabase configuration
const SUPABASE_URL: &str = "https://wqutewgfxtucshqwzecj.supabase.co";
const SUPABASE_ANON_KEY: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6IndxdXRld2dmeHR1Y3NocXd6ZWNqIiwicm9sZSI6ImFub24iLCJpYXQiOjE3NjkzNzQ4MDgsImV4cCI6MjA4NDk1MDgwOH0.Wx2Yd5ONcorGqLTeNsMgeOrkBQBwN8Kjag_rBXX60O8";

#[derive(Debug, Error)]
pub enum CloudError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("Failed to parse response: {0}")]
    Parse(String),
    #[error("Server error: {0}")]
    Server(String),
}

/// A summary of a community benchmark run for browsing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityRun {
    pub id: String,
    pub display_name: String,
    pub cpu_name: String,
    pub cpu_cores: i32,
    pub cpu_threads: i32,
    pub memory_gb: f64,
    pub os_name: String,
    pub storage_type: Option<String>,
    pub uploaded_at: DateTime<Utc>,
}

/// Filter options for browsing community results
#[derive(Debug, Clone, Default)]
pub struct BrowseFilter {
    pub cpu_contains: Option<String>,
    pub os_name: Option<String>,
    pub min_memory_gb: Option<f64>,
    pub limit: usize,
}

impl BrowseFilter {
    pub fn new() -> Self {
        Self {
            limit: 50,
            ..Default::default()
        }
    }

    pub fn with_cpu(mut self, cpu: &str) -> Self {
        self.cpu_contains = Some(cpu.to_string());
        self
    }

    pub fn with_os(mut self, os: &str) -> Self {
        self.os_name = Some(os.to_string());
        self
    }

    pub fn with_min_memory(mut self, gb: f64) -> Self {
        self.min_memory_gb = Some(gb);
        self
    }

    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }
}

/// Database row for benchmark runs
#[derive(Debug, Serialize, Deserialize)]
struct BenchmarkRunRow {
    id: String,
    display_name: String,
    machine_name: String,
    run_timestamp: DateTime<Utc>,
    cpu_name: String,
    cpu_cores: i32,
    cpu_threads: i32,
    memory_gb: f64,
    os_name: String,
    storage_type: Option<String>,
    results: serde_json::Value,
    system_info: serde_json::Value,
    uploaded_at: DateTime<Utc>,
}

/// Upload payload for creating a new benchmark run
#[derive(Debug, Serialize)]
struct UploadPayload {
    display_name: String,
    machine_name: String,
    run_timestamp: DateTime<Utc>,
    cpu_name: String,
    cpu_cores: i32,
    cpu_threads: i32,
    memory_gb: f64,
    os_name: String,
    storage_type: Option<String>,
    results: serde_json::Value,
    system_info: serde_json::Value,
}

/// A histogram bucket for distribution visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistogramBucket {
    pub bucket_start: f64,
    pub bucket_end: f64,
    pub count: i64,
}

/// Statistics for a single test across all community runs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestStatistics {
    pub test_id: String,
    pub test_name: String,
    pub unit: String,
    pub sample_count: i64,
    pub min_value: f64,
    pub max_value: f64,
    pub mean_value: f64,
    pub std_dev: f64,
    pub p10: f64,
    pub p25: f64,
    pub p50: f64,
    pub p75: f64,
    pub p90: f64,
    pub histogram_buckets: Vec<HistogramBucket>,
}

/// Percentile rank for a user's test result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PercentileRank {
    pub test_id: String,
    pub test_name: String,
    pub unit: String,
    pub user_value: f64,
    pub percentile_rank: f64,
    pub beats_count: i64,
    pub total_count: i64,
    pub is_higher_better: bool,
}

/// Client for interacting with the community benchmark database
pub struct CloudClient {
    client: Client,
}

impl CloudClient {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    /// Browse community benchmark results with optional filters
    pub fn browse(&self, filter: &BrowseFilter) -> Result<Vec<CommunityRun>, CloudError> {
        let mut url = format!(
            "{}/rest/v1/benchmark_runs?select=id,display_name,cpu_name,cpu_cores,cpu_threads,memory_gb,os_name,storage_type,uploaded_at&order=uploaded_at.desc&limit={}",
            SUPABASE_URL, filter.limit
        );

        // Add filters
        if let Some(ref cpu) = filter.cpu_contains {
            url.push_str(&format!("&cpu_name=ilike.*{}*", urlencoding::encode(cpu)));
        }
        if let Some(ref os) = filter.os_name {
            url.push_str(&format!("&os_name=ilike.*{}*", urlencoding::encode(os)));
        }
        if let Some(min_mem) = filter.min_memory_gb {
            url.push_str(&format!("&memory_gb=gte.{}", min_mem));
        }

        let response = self
            .client
            .get(&url)
            .header("apikey", SUPABASE_ANON_KEY)
            .header("Authorization", format!("Bearer {}", SUPABASE_ANON_KEY))
            .send()?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            return Err(CloudError::Server(format!("{}: {}", status, body)));
        }

        let runs: Vec<CommunityRun> = response.json()?;
        Ok(runs)
    }

    /// Fetch full details of a specific benchmark run
    pub fn fetch(&self, id: &str) -> Result<BenchmarkRun, CloudError> {
        let url = format!(
            "{}/rest/v1/benchmark_runs?id=eq.{}&select=*",
            SUPABASE_URL, id
        );

        let response = self
            .client
            .get(&url)
            .header("apikey", SUPABASE_ANON_KEY)
            .header("Authorization", format!("Bearer {}", SUPABASE_ANON_KEY))
            .send()?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            return Err(CloudError::Server(format!("{}: {}", status, body)));
        }

        let rows: Vec<BenchmarkRunRow> = response.json()?;
        let row = rows
            .into_iter()
            .next()
            .ok_or_else(|| CloudError::Parse("Run not found".to_string()))?;

        // Convert to BenchmarkRun
        let results = serde_json::from_value(row.results)
            .map_err(|e| CloudError::Parse(format!("Failed to parse results: {}", e)))?;
        let system_info = serde_json::from_value(row.system_info)
            .map_err(|e| CloudError::Parse(format!("Failed to parse system_info: {}", e)))?;

        Ok(BenchmarkRun {
            id: uuid::Uuid::parse_str(&row.id)
                .map_err(|e| CloudError::Parse(format!("Invalid UUID: {}", e)))?,
            timestamp: row.run_timestamp,
            machine_name: row.machine_name,
            notes: None,
            tags: Vec::new(),
            system_info,
            results,
            remote_id: Some(row.id),
            uploaded_at: Some(row.uploaded_at),
        })
    }

    /// Delete an uploaded benchmark run from the community database
    pub fn delete(&self, remote_id: &str) -> Result<(), CloudError> {
        let url = format!(
            "{}/rest/v1/benchmark_runs?id=eq.{}",
            SUPABASE_URL, remote_id
        );

        let response = self
            .client
            .delete(&url)
            .header("apikey", SUPABASE_ANON_KEY)
            .header("Authorization", format!("Bearer {}", SUPABASE_ANON_KEY))
            .send()?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            return Err(CloudError::Server(format!("{}: {}", status, body)));
        }

        Ok(())
    }

    /// Upload a local benchmark run to the community database
    pub fn upload(&self, run: &BenchmarkRun, display_name: &str) -> Result<String, CloudError> {
        let url = format!("{}/rest/v1/benchmark_runs", SUPABASE_URL);

        // Determine storage type from system info
        let storage_type = run
            .system_info
            .storage
            .first()
            .map(|s| match s.device_type {
                StorageType::NVMe => "NVMe",
                StorageType::SSD => "SSD",
                StorageType::HDD => "HDD",
                StorageType::Unknown => "Unknown",
            })
            .map(|s| s.to_string());

        let payload = UploadPayload {
            display_name: display_name.to_string(),
            machine_name: run.machine_name.clone(),
            run_timestamp: run.timestamp,
            cpu_name: run.system_info.cpu.name.clone(),
            cpu_cores: run.system_info.cpu.cores as i32,
            cpu_threads: run.system_info.cpu.threads as i32,
            memory_gb: run.system_info.memory.total_gb(),
            os_name: run.system_info.os.name.clone(),
            storage_type,
            results: serde_json::to_value(&run.results)
                .map_err(|e| CloudError::Parse(format!("Failed to serialize results: {}", e)))?,
            system_info: serde_json::to_value(&run.system_info)
                .map_err(|e| CloudError::Parse(format!("Failed to serialize system_info: {}", e)))?,
        };

        let response = self
            .client
            .post(&url)
            .header("apikey", SUPABASE_ANON_KEY)
            .header("Authorization", format!("Bearer {}", SUPABASE_ANON_KEY))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=representation")
            .json(&payload)
            .send()?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            return Err(CloudError::Server(format!("{}: {}", status, body)));
        }

        // Parse response to get the created ID
        let created: Vec<BenchmarkRunRow> = response.json()?;
        let id = created
            .into_iter()
            .next()
            .map(|r| r.id)
            .ok_or_else(|| CloudError::Parse("No ID returned".to_string()))?;

        Ok(id)
    }

    /// Fetch test statistics with histogram buckets for all tests
    pub fn fetch_statistics(&self) -> Result<Vec<TestStatistics>, CloudError> {
        let url = format!("{}/rest/v1/rpc/get_test_statistics", SUPABASE_URL);

        let response = self
            .client
            .post(&url)
            .header("apikey", SUPABASE_ANON_KEY)
            .header("Authorization", format!("Bearer {}", SUPABASE_ANON_KEY))
            .header("Content-Type", "application/json")
            .body("{}")
            .send()?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            return Err(CloudError::Server(format!("{}: {}", status, body)));
        }

        let stats: Vec<TestStatistics> = response.json()?;
        Ok(stats)
    }

    /// Fetch percentile rank for a specific run
    pub fn fetch_percentile_rank(&self, run_id: &str) -> Result<Vec<PercentileRank>, CloudError> {
        let url = format!("{}/rest/v1/rpc/get_percentile_rank", SUPABASE_URL);

        let body = serde_json::json!({ "run_id": run_id });

        let response = self
            .client
            .post(&url)
            .header("apikey", SUPABASE_ANON_KEY)
            .header("Authorization", format!("Bearer {}", SUPABASE_ANON_KEY))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            return Err(CloudError::Server(format!("{}: {}", status, body)));
        }

        let ranks: Vec<PercentileRank> = response.json()?;
        Ok(ranks)
    }
}

impl Default for CloudClient {
    fn default() -> Self {
        Self::new()
    }
}
