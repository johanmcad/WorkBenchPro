use std::process::Command;

use anyhow::Result;

use crate::benchmarks::{Benchmark, Category, ProgressCallback};
use crate::core::Timer;
use crate::models::{TestDetails, TestResult};

/// Network operations benchmark
/// Tests DNS resolution and network queries
pub struct NetworkBenchmark;

impl NetworkBenchmark {
    pub fn new() -> Self {
        Self
    }
}

impl Default for NetworkBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl Benchmark for NetworkBenchmark {
    fn id(&self) -> &'static str {
        "network"
    }

    fn name(&self) -> &'static str {
        "Network Operations"
    }

    fn description(&self) -> &'static str {
        "DNS resolution, network adapter queries, routing table"
    }

    fn category(&self) -> Category {
        Category::Responsiveness
    }

    fn estimated_duration_secs(&self) -> u32 {
        30
    }

    fn run(&self, progress: &dyn ProgressCallback) -> Result<TestResult> {
        progress.update(0.05, "Preparing network benchmark...");

        let is_windows = cfg!(target_os = "windows");

        let mut dns_times: Vec<f64> = Vec::new();
        let mut adapter_times: Vec<f64> = Vec::new();
        let mut route_times: Vec<f64> = Vec::new();

        // DNS hostnames to resolve (common, reliable hosts)
        let hostnames = [
            "localhost",
            "google.com",
            "microsoft.com",
            "github.com",
        ];

        // Test 1: DNS resolution
        progress.update(0.1, "Testing DNS resolution...");

        for round in 0..3 {
            if progress.is_cancelled() {
                return Err(anyhow::anyhow!("Cancelled"));
            }

            for host in &hostnames {
                let timer = Timer::new();
                if is_windows {
                    let _ = Command::new("nslookup")
                        .arg(host)
                        .output();
                } else {
                    let _ = Command::new("host")
                        .arg(host)
                        .output();
                }
                dns_times.push(timer.elapsed_secs() * 1000.0);
            }

            progress.update(
                0.1 + (round as f32 / 3.0) * 0.3,
                &format!("DNS resolution round {}/3...", round + 1),
            );
        }

        // Test 2: Network adapter information
        progress.update(0.4, "Querying network adapters...");

        for round in 0..5 {
            if progress.is_cancelled() {
                return Err(anyhow::anyhow!("Cancelled"));
            }

            let timer = Timer::new();
            if is_windows {
                let _ = Command::new("ipconfig")
                    .arg("/all")
                    .output();
            } else {
                let _ = Command::new("ip")
                    .args(["addr", "show"])
                    .output();
            }
            adapter_times.push(timer.elapsed_secs() * 1000.0);

            progress.update(
                0.4 + (round as f32 / 5.0) * 0.3,
                &format!("Adapter query round {}/5...", round + 1),
            );
        }

        // Test 3: Routing table
        progress.update(0.7, "Querying routing table...");

        for round in 0..5 {
            if progress.is_cancelled() {
                return Err(anyhow::anyhow!("Cancelled"));
            }

            let timer = Timer::new();
            if is_windows {
                let _ = Command::new("route")
                    .arg("print")
                    .output();
            } else {
                let _ = Command::new("ip")
                    .args(["route", "show"])
                    .output();
            }
            route_times.push(timer.elapsed_secs() * 1000.0);

            progress.update(
                0.7 + (round as f32 / 5.0) * 0.25,
                &format!("Route query round {}/5...", round + 1),
            );
        }

        // Calculate statistics
        let avg_dns = dns_times.iter().sum::<f64>() / dns_times.len() as f64;
        let avg_adapter = adapter_times.iter().sum::<f64>() / adapter_times.len() as f64;
        let avg_route = route_times.iter().sum::<f64>() / route_times.len() as f64;
        let avg_combined = (avg_dns + avg_adapter + avg_route) / 3.0;

        // Score: faster is better
        let score = if avg_combined < 50.0 {
            300
        } else if avg_combined < 100.0 {
            250
        } else if avg_combined < 200.0 {
            200
        } else if avg_combined < 500.0 {
            150
        } else {
            100
        };

        let all_times: Vec<f64> = [dns_times, adapter_times, route_times].concat();
        let min = all_times.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = all_times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        progress.update(1.0, "Complete");

        Ok(TestResult {
            test_id: self.id().to_string(),
            name: self.name().to_string(),
            description: format!(
                "{} (dns: {:.0}ms, adapter: {:.0}ms, route: {:.0}ms)",
                self.description(),
                avg_dns,
                avg_adapter,
                avg_route
            ),
            value: avg_combined,
            unit: "ms".to_string(),
            score,
            max_score: 300,
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
