use std::time::Instant;

/// High-precision timer for benchmarking
pub struct Timer {
    start: Instant,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    pub fn start(&mut self) {
        self.start = Instant::now();
    }

    pub fn elapsed_ns(&self) -> u64 {
        self.start.elapsed().as_nanos() as u64
    }

    pub fn elapsed_us(&self) -> f64 {
        self.start.elapsed().as_nanos() as f64 / 1000.0
    }

    pub fn elapsed_ms(&self) -> f64 {
        self.start.elapsed().as_nanos() as f64 / 1_000_000.0
    }

    pub fn elapsed_secs(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

/// Get current time in nanoseconds using high-precision counter
#[cfg(windows)]
pub fn precise_time_ns() -> u64 {
    use windows::Win32::System::Performance::{QueryPerformanceCounter, QueryPerformanceFrequency};

    let mut counter = 0i64;
    let mut frequency = 0i64;
    unsafe {
        QueryPerformanceCounter(&mut counter).ok();
        QueryPerformanceFrequency(&mut frequency).ok();
    }
    if frequency == 0 {
        return 0;
    }
    ((counter as u128 * 1_000_000_000) / frequency as u128) as u64
}

#[cfg(not(windows))]
pub fn precise_time_ns() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0)
}

/// Measure the execution time of a closure in nanoseconds
pub fn measure_ns<F, T>(f: F) -> (T, u64)
where
    F: FnOnce() -> T,
{
    let timer = Timer::new();
    let result = f();
    (result, timer.elapsed_ns())
}

/// Measure the execution time of a closure in milliseconds
pub fn measure_ms<F, T>(f: F) -> (T, f64)
where
    F: FnOnce() -> T,
{
    let timer = Timer::new();
    let result = f();
    (result, timer.elapsed_ms())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_timer_basic() {
        let timer = Timer::new();
        sleep(Duration::from_millis(10));
        let elapsed = timer.elapsed_ms();
        assert!(elapsed >= 9.0 && elapsed < 50.0);
    }

    #[test]
    fn test_measure_ns() {
        let (result, ns) = measure_ns(|| {
            sleep(Duration::from_millis(5));
            42
        });
        assert_eq!(result, 42);
        assert!(ns >= 4_000_000); // At least 4ms
    }
}
