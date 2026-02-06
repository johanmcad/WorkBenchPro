use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[cfg(feature = "debug-logging")]
use std::time::Instant;

use crate::benchmarks::{Benchmark, BenchmarkConfig, Category, ProgressCallback};
use crate::models::{BenchmarkRun, CategoryResults, TestResult};

use super::SystemInfoCollector;

#[cfg(feature = "debug-logging")]
use tracing::{debug, info, warn, error};

/// Messages sent from benchmark thread to UI
#[derive(Debug, Clone)]
pub enum BenchmarkMessage {
    Progress {
        benchmark_id: String,
        overall_progress: f32,
        test_progress: f32,
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

    /// Start running benchmarks
    pub fn start(
        &mut self,
        benchmarks: Vec<Box<dyn Benchmark>>,
    ) -> Receiver<BenchmarkMessage> {
        #[cfg(feature = "debug-logging")]
        {
            info!("BenchmarkRunner::start() called");
            info!("Benchmarks queued: {}", benchmarks.len());
            for (i, b) in benchmarks.iter().enumerate() {
                debug!("  [{}] {} - {}", i + 1, b.id(), b.name());
            }
        }

        let (tx, rx) = channel();
        self.sender = Some(tx.clone());

        *self.cancel_flag.lock().unwrap() = false;
        let cancel_flag = Arc::clone(&self.cancel_flag);

        let handle = thread::spawn(move || {
            Self::run_benchmarks(tx, benchmarks, cancel_flag);
        });

        self.handle = Some(handle);
        rx
    }

    /// Cancel the current benchmark run
    pub fn cancel(&self) {
        #[cfg(feature = "debug-logging")]
        warn!("BenchmarkRunner::cancel() called - requesting cancellation");

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
        cancel_flag: Arc<Mutex<bool>>,
    ) {
        #[cfg(feature = "debug-logging")]
        let run_start = Instant::now();

        #[cfg(feature = "debug-logging")]
        info!("========================================");
        #[cfg(feature = "debug-logging")]
        info!("BENCHMARK RUN STARTED");
        #[cfg(feature = "debug-logging")]
        info!("Total benchmarks to run: {}", benchmarks.len());
        #[cfg(feature = "debug-logging")]
        info!("========================================");

        // Collect system info
        #[cfg(feature = "debug-logging")]
        info!("Collecting system information...");
        let system_info = SystemInfoCollector::collect();
        let machine_name = system_info.hostname.clone();

        #[cfg(feature = "debug-logging")]
        {
            info!("System Info Collected:");
            info!("  Machine name: {}", machine_name);
            info!("  CPU: {}", system_info.cpu.name);
            info!("  Cores: {} / Threads: {}", system_info.cpu.cores, system_info.cpu.threads);
            info!("  RAM: {:.1} GB", system_info.memory.total_gb());
            info!("  OS: {} {}", system_info.os.name, system_info.os.version);
        }

        let mut run = BenchmarkRun::new(machine_name, system_info);
        let mut results = CategoryResults::default();

        let total = benchmarks.len();

        // Use default benchmark config (Quick preset)
        let benchmark_config = BenchmarkConfig::default();

        #[cfg(feature = "debug-logging")]
        {
            info!("Benchmark Configuration:");
            info!("  Iterations: {}", benchmark_config.iterations);
            debug!("  Config: {:?}", benchmark_config);
        }

        for (idx, benchmark) in benchmarks.into_iter().enumerate() {
            // Check for cancellation
            if *cancel_flag.lock().unwrap() {
                #[cfg(feature = "debug-logging")]
                warn!("Benchmark run CANCELLED by user");
                let _ = tx.send(BenchmarkMessage::Cancelled);
                return;
            }

            let overall_progress = idx as f32 / total as f32;

            #[cfg(feature = "debug-logging")]
            let bench_start = Instant::now();

            #[cfg(feature = "debug-logging")]
            {
                info!("----------------------------------------");
                info!("BENCHMARK [{}/{}]: {} ({})", idx + 1, total, benchmark.name(), benchmark.id());
                info!("  Category: {:?}", benchmark.category());
                info!("  Description: {}", benchmark.description());
                info!("  Starting...");
            }

            let progress_callback = ChannelProgressCallback {
                tx: tx.clone(),
                benchmark_id: benchmark.id().to_string(),
                cancel_flag: Arc::clone(&cancel_flag),
                overall_progress,
                #[cfg(feature = "debug-logging")]
                benchmark_name: benchmark.name().to_string(),
            };

            // Send initial progress
            let _ = tx.send(BenchmarkMessage::Progress {
                benchmark_id: benchmark.id().to_string(),
                overall_progress,
                test_progress: 0.0,
                message: format!("Running {} ({}/{})", benchmark.name(), idx + 1, total),
            });

            // Run the benchmark
            match benchmark.run(&progress_callback, &benchmark_config) {
                Ok(result) => {
                    #[cfg(feature = "debug-logging")]
                    {
                        let elapsed = bench_start.elapsed();
                        info!("  COMPLETED in {:.2}s", elapsed.as_secs_f64());
                        info!("  Result: {} = {:.4} {}", result.name, result.value, result.unit);
                        if let Some(ref details) = Some(&result.details) {
                            info!("  Details:");
                            info!("    Iterations: {}", details.iterations);
                            info!("    Duration: {:.3}s", details.duration_secs);
                            info!("    Min: {:.4}", details.min);
                            info!("    Max: {:.4}", details.max);
                            info!("    Mean: {:.4}", details.mean);
                            info!("    Median: {:.4}", details.median);
                            info!("    Std Dev: {:.4}", details.std_dev);
                            if let Some(ref p) = details.percentiles {
                                debug!("    Percentiles: p50={:.4} p75={:.4} p90={:.4} p95={:.4} p99={:.4}",
                                    p.p50, p.p75, p.p90, p.p95, p.p99);
                            }
                        }
                    }

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
                    #[cfg(feature = "debug-logging")]
                    {
                        let elapsed = bench_start.elapsed();
                        error!("  FAILED after {:.2}s", elapsed.as_secs_f64());
                        error!("  Error: {}", e);
                    }
                    let _ = tx.send(BenchmarkMessage::Error {
                        error: format!("Benchmark {} failed: {}", benchmark.name(), e),
                    });
                }
            }

            // Add delay between tests to avoid triggering AV behavioral heuristics
            // that detect rapid suspicious activity patterns
            if idx < total - 1 {
                #[cfg(feature = "debug-logging")]
                debug!("  Waiting 2s before next test (AV evasion delay)...");
                thread::sleep(Duration::from_secs(2));
            }
        }

        run.results = results;

        #[cfg(feature = "debug-logging")]
        {
            let total_elapsed = run_start.elapsed();
            info!("========================================");
            info!("BENCHMARK RUN COMPLETED");
            info!("Total time: {:.2}s", total_elapsed.as_secs_f64());
            info!("Results summary:");
            info!("  Project Operations: {} tests", run.results.project_operations.len());
            info!("  Build Performance: {} tests", run.results.build_performance.len());
            info!("  Responsiveness: {} tests", run.results.responsiveness.len());
            info!("========================================");
        }

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
    overall_progress: f32,
    #[cfg(feature = "debug-logging")]
    benchmark_name: String,
}

impl ProgressCallback for ChannelProgressCallback {
    fn update(&self, progress: f32, message: &str) {
        #[cfg(feature = "debug-logging")]
        debug!("    [{}] Progress: {:.1}% - {}", self.benchmark_name, progress * 100.0, message);

        let _ = self.tx.send(BenchmarkMessage::Progress {
            benchmark_id: self.benchmark_id.clone(),
            overall_progress: self.overall_progress,
            test_progress: progress,
            message: message.to_string(),
        });
    }

    fn is_cancelled(&self) -> bool {
        let cancelled = *self.cancel_flag.lock().unwrap();
        #[cfg(feature = "debug-logging")]
        if cancelled {
            warn!("    [{}] Cancellation requested", self.benchmark_name);
        }
        cancelled
    }
}
