use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

use anyhow::Result;

use crate::benchmarks::{Benchmark, Category, ProgressCallback};
use crate::core::Timer;
use crate::models::{TestDetails, TestResult};

/// Thread wake latency benchmark
/// Measures time to wake a sleeping thread 1,000 times
pub struct ThreadWakeBenchmark;

impl ThreadWakeBenchmark {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ThreadWakeBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl Benchmark for ThreadWakeBenchmark {
    fn id(&self) -> &'static str {
        "thread_wake"
    }

    fn name(&self) -> &'static str {
        "Thread Wake Latency"
    }

    fn description(&self) -> &'static str {
        "Signal sleeping thread 1,000 times - simulates async operations"
    }

    fn category(&self) -> Category {
        Category::Responsiveness
    }

    fn estimated_duration_secs(&self) -> u32 {
        10
    }

    fn run(&self, progress: &dyn ProgressCallback) -> Result<TestResult> {
        let num_wakes: usize = 1000;
        let mut wake_times_us: Vec<f64> = Vec::with_capacity(num_wakes);

        progress.update(0.0, "Setting up thread wake test...");

        // Shared state
        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let wake_time = Arc::new(AtomicU64::new(0));
        let should_exit = Arc::new(AtomicBool::new(false));

        // Clone for worker thread
        let pair_clone = Arc::clone(&pair);
        let wake_time_clone = Arc::clone(&wake_time);
        let should_exit_clone = Arc::clone(&should_exit);

        // Spawn worker thread that waits to be woken
        let worker = thread::spawn(move || {
            let (lock, cvar) = &*pair_clone;

            loop {
                let mut started = lock.lock().unwrap();

                // Wait for signal
                while !*started && !should_exit_clone.load(Ordering::Relaxed) {
                    started = cvar.wait(started).unwrap();
                }

                if should_exit_clone.load(Ordering::Relaxed) {
                    break;
                }

                // Record wake time
                let now = std::time::Instant::now();
                wake_time_clone.store(now.elapsed().as_nanos() as u64, Ordering::SeqCst);

                // Reset for next iteration
                *started = false;
            }
        });

        // Give thread time to start and enter wait state
        thread::sleep(std::time::Duration::from_millis(10));

        progress.update(0.1, "Measuring wake latencies...");

        for i in 0..num_wakes {
            if progress.is_cancelled() {
                should_exit.store(true, Ordering::Relaxed);
                let (lock, cvar) = &*pair;
                {
                    let mut started = lock.lock().unwrap();
                    *started = true;
                }
                cvar.notify_one();
                let _ = worker.join();
                return Err(anyhow::anyhow!("Cancelled"));
            }

            let (lock, cvar) = &*pair;

            // Record start time and signal the thread
            let timer = Timer::new();

            {
                let mut started = lock.lock().unwrap();
                *started = true;
            }
            cvar.notify_one();

            // Wait a bit for the worker to process
            thread::sleep(std::time::Duration::from_micros(100));

            let elapsed_us = timer.elapsed_secs() * 1_000_000.0;
            wake_times_us.push(elapsed_us);

            // Wait for worker to reset
            loop {
                let started = lock.lock().unwrap();
                if !*started {
                    break;
                }
                drop(started);
                thread::sleep(std::time::Duration::from_micros(10));
            }

            if i % 100 == 0 {
                progress.update(
                    0.1 + (i as f32 / num_wakes as f32) * 0.85,
                    &format!("Wake test {}/{}...", i + 1, num_wakes),
                );
            }
        }

        // Cleanup: signal thread to exit
        should_exit.store(true, Ordering::Relaxed);
        {
            let (lock, cvar) = &*pair;
            let mut started = lock.lock().unwrap();
            *started = true;
            cvar.notify_one();
            drop(started);
        }
        let _ = worker.join();

        // Calculate statistics
        wake_times_us.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let count = wake_times_us.len();
        if count == 0 {
            return Err(anyhow::anyhow!("No measurements collected"));
        }

        let min = wake_times_us[0];
        let max = wake_times_us[count - 1];
        let sum: f64 = wake_times_us.iter().sum();
        let mean = sum / count as f64;
        let median = wake_times_us[count / 2];

        let variance: f64 = wake_times_us
            .iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>()
            / count as f64;
        let std_dev = variance.sqrt();

        progress.update(1.0, "Complete");

        Ok(TestResult {
            test_id: self.id().to_string(),
            name: self.name().to_string(),
            description: self.description().to_string(),
            value: mean,
            unit: "us".to_string(),
            details: TestDetails {
                iterations: count as u32,
                duration_secs: sum / 1_000_000.0,
                min,
                max,
                mean,
                median,
                std_dev,
                percentiles: Some(crate::models::Percentiles {
                    p50: median,
                    p75: wake_times_us[((count as f64) * 0.75) as usize],
                    p90: wake_times_us[((count as f64) * 0.90) as usize],
                    p95: wake_times_us[((count as f64) * 0.95) as usize],
                    p99: wake_times_us[((count as f64) * 0.99) as usize],
                    p999: wake_times_us[((count as f64) * 0.999).min(count as f64 - 1.0) as usize],
                }),
            },
        })
    }
}
