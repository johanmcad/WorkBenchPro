# Data Models

## System Information

```rust
pub struct SystemInfo {
    pub hostname: String,
    pub cpu: CpuInfo,
    pub memory: MemoryInfo,
    pub storage: Vec<StorageInfo>,
    pub gpu: Option<GpuInfo>,
    pub os: OsInfo,
}

pub struct CpuInfo {
    pub name: String,
    pub vendor: String,
    pub cores: u32,
    pub threads: u32,
    pub base_frequency_mhz: u32,
    pub max_frequency_mhz: Option<u32>,
    pub cache_l3_kb: Option<u32>,
}

pub struct MemoryInfo {
    pub total_bytes: u64,
    pub speed_mhz: Option<u32>,
    pub memory_type: Option<String>,
}

pub struct StorageInfo {
    pub name: String,
    pub device_type: StorageType,
    pub capacity_bytes: u64,
}

pub enum StorageType {
    NVMe,
    SSD,
    HDD,
    Unknown,
}

pub struct GpuInfo {
    pub name: String,
    pub vendor: String,
    pub vram_bytes: Option<u64>,
    pub driver_version: Option<String>,
}

pub struct OsInfo {
    pub name: String,
    pub version: String,
    pub build: Option<String>,
}
```

## Benchmark Results

```rust
pub struct BenchmarkRun {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub machine_name: String,
    pub notes: Option<String>,
    pub tags: Vec<String>,
    pub system_info: SystemInfo,
    pub results: CategoryResults,
    pub scores: Scores,
}

pub struct CategoryResults {
    pub project_operations: Vec<TestResult>,
    pub build_performance: Vec<TestResult>,
    pub responsiveness: Vec<TestResult>,
    pub graphics: Option<Vec<TestResult>>,
}

pub struct TestResult {
    pub test_id: String,
    pub name: String,
    pub description: String,
    pub value: f64,
    pub unit: String,
    pub score: u32,
    pub max_score: u32,
    pub details: TestDetails,
}

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

pub struct Percentiles {
    pub p50: f64,
    pub p75: f64,
    pub p90: f64,
    pub p95: f64,
    pub p99: f64,
    pub p999: f64,
}
```

## Scores

```rust
pub struct Scores {
    pub overall: u32,
    pub overall_max: u32,
    pub rating: Rating,
    pub categories: CategoryScores,
}

pub struct CategoryScores {
    pub project_operations: CategoryScore,
    pub build_performance: CategoryScore,
    pub responsiveness: CategoryScore,
    pub graphics: Option<CategoryScore>,
}

pub struct CategoryScore {
    pub score: u32,
    pub max_score: u32,
    pub percentage: f64,
    pub rating: Rating,
}

#[derive(Clone, Copy)]
pub enum Rating {
    Excellent,   // 90-100%
    Good,        // 70-90%
    Acceptable,  // 50-70%
    Poor,        // 30-50%
    Inadequate,  // <30%
}

impl Rating {
    pub fn from_percentage(pct: f64) -> Self {
        match pct {
            p if p >= 90.0 => Rating::Excellent,
            p if p >= 70.0 => Rating::Good,
            p if p >= 50.0 => Rating::Acceptable,
            p if p >= 30.0 => Rating::Poor,
            _ => Rating::Inadequate,
        }
    }
    
    pub fn label(&self) -> &'static str {
        match self {
            Rating::Excellent => "Excellent",
            Rating::Good => "Good",
            Rating::Acceptable => "Acceptable",
            Rating::Poor => "Poor",
            Rating::Inadequate => "Inadequate",
        }
    }
}
```

## Benchmark Trait

```rust
pub trait Benchmark: Send + Sync {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn category(&self) -> Category;
    fn estimated_duration_secs(&self) -> u32;
    fn run(&self, progress: &dyn ProgressCallback) -> Result<TestResult>;
}

pub trait ProgressCallback: Send + Sync {
    fn update(&self, progress: f32, message: &str);
    fn is_cancelled(&self) -> bool;
}

pub enum Category {
    ProjectOperations,
    BuildPerformance,
    Responsiveness,
    Graphics,
}
```
