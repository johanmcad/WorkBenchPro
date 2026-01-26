use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use anyhow::Result;

use crate::benchmarks::{Benchmark, Category, ProgressCallback};
use crate::models::{BenchmarkRun, CategoryResults, TestResult};

use super::SystemInfoCollector;

/// Messages sent from benchmark thread to UI
#[derive(Debug, Clone)]
pub enum BenchmarkMessage {
    Progress {
        benchmark_id: String,
        progress: f32,
        message: String,
    },
    TestComplete {
        result: TestResult,
    },
    AllComplete {
        run: Box<BenchmarkRun>,
    },
    Error {
        error: String,
    },
    Cancelled,
}

/// Configuration for a benchmark run
#[derive(Debug, Clone, Default)]
pub struct RunConfig {
    pub machine_name: String,
    /// Skip synthetic benchmarks (CPU, disk, latency, memory microbenchmarks)
    /// and only run real application benchmarks
    pub skip_synthetic: bool,
}

/// Runs benchmarks in a background thread
pub struct BenchmarkRunner {
    sender: Option<Sender<BenchmarkMessage>>,
    cancel_flag: Arc<Mutex<bool>>,
    handle: Option<thread::JoinHandle<()>>,
}

impl BenchmarkRunner {
    pub fn new() -> Self {
        Self {
            sender: None,
            cancel_flag: Arc::new(Mutex::new(false)),
            handle: None,
        }
    }

    /// Start running benchmarks with the given configuration
    pub fn start(
        &mut self,
        benchmarks: Vec<Box<dyn Benchmark>>,
        config: RunConfig,
    ) -> Receiver<BenchmarkMessage> {
        let (tx, rx) = channel();
        self.sender = Some(tx.clone());

        *self.cancel_flag.lock().unwrap() = false;
        let cancel_flag = Arc::clone(&self.cancel_flag);

        let handle = thread::spawn(move || {
            Self::run_benchmarks(tx, benchmarks, config, cancel_flag);
        });

        self.handle = Some(handle);
        rx
    }

    /// Cancel the current benchmark run
    pub fn cancel(&self) {
        if let Ok(mut flag) = self.cancel_flag.lock() {
            *flag = true;
        }
    }

    /// Check if benchmarks are currently running
    pub fn is_running(&self) -> bool {
        self.handle
            .as_ref()
            .map(|h| !h.is_finished())
            .unwrap_or(false)
    }

    fn run_benchmarks(
        tx: Sender<BenchmarkMessage>,
        benchmarks: Vec<Box<dyn Benchmark>>,
        config: RunConfig,
        cancel_flag: Arc<Mutex<bool>>,
    ) {
        // Collect system info
        let system_info = SystemInfoCollector::collect();
        let machine_name = if config.machine_name.is_empty() {
            system_info.hostname.clone()
        } else {
            config.machine_name.clone()
        };

        let mut run = BenchmarkRun::new(machine_name, system_info);
        let mut results = CategoryResults::default();

        let total = benchmarks.len();

        for (idx, benchmark) in benchmarks.into_iter().enumerate() {
            // Check for cancellation
            if *cancel_flag.lock().unwrap() {
                let _ = tx.send(BenchmarkMessage::Cancelled);
                return;
            }

            let progress_callback = ChannelProgressCallback {
                tx: tx.clone(),
                benchmark_id: benchmark.id().to_string(),
                cancel_flag: Arc::clone(&cancel_flag),
            };

            // Send initial progress
            let _ = tx.send(BenchmarkMessage::Progress {
                benchmark_id: benchmark.id().to_string(),
                progress: idx as f32 / total as f32,
                message: format!("Running {} ({}/{})", benchmark.name(), idx + 1, total),
            });

            // Run the benchmark
            match benchmark.run(&progress_callback) {
                Ok(result) => {
                    // Categorize the result
                    match benchmark.category() {
                        Category::ProjectOperations => {
                            results.project_operations.push(result.clone());
                        }
                        Category::BuildPerformance => {
                            results.build_performance.push(result.clone());
                        }
                        Category::Responsiveness => {
                            results.responsiveness.push(result.clone());
                        }
                    }

                    let _ = tx.send(BenchmarkMessage::TestComplete { result });
                }
                Err(e) => {
                    let _ = tx.send(BenchmarkMessage::Error {
                        error: format!("Benchmark {} failed: {}", benchmark.name(), e),
                    });
                }
            }
        }

        run.results = results;

        let _ = tx.send(BenchmarkMessage::AllComplete { run: Box::new(run) });
    }
}

impl Default for BenchmarkRunner {
    fn default() -> Self {
        Self::new()
    }
}

/// Progress callback that sends updates via channel
struct ChannelProgressCallback {
    tx: Sender<BenchmarkMessage>,
    benchmark_id: String,
    cancel_flag: Arc<Mutex<bool>>,
}

impl ProgressCallback for ChannelProgressCallback {
    fn update(&self, progress: f32, message: &str) {
        let _ = self.tx.send(BenchmarkMessage::Progress {
            benchmark_id: self.benchmark_id.clone(),
            progress,
            message: message.to_string(),
        });
    }

    fn is_cancelled(&self) -> bool {
        *self.cancel_flag.lock().unwrap()
    }
}
