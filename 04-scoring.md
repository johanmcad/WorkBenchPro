# Scoring System

## Overall: 10,000 points maximum

| Category | Max Points |
|----------|------------|
| Project Operations | 2,500 |
| Build Performance | 2,500 |
| Responsiveness | 2,500 |
| Graphics | 2,500 |

## Ratings

| Rating | % | Score | Meaning |
|--------|---|-------|---------|
| Excellent | 90%+ | 9,000+ | Premium workstation |
| Good | 70-90% | 7,000-8,999 | Smooth experience |
| Acceptable | 50-70% | 5,000-6,999 | Some waiting |
| Poor | 30-50% | 3,000-4,999 | Noticeable slowdowns |
| Inadequate | <30% | <3,000 | Significant impact |

## Scoring Functions

```rust
// File enumeration: files/sec -> score (max 500)
pub fn file_enumeration_score(files_per_sec: f64) -> u32 {
    match files_per_sec {
        x if x >= 60_000.0 => 500,
        x if x >= 45_000.0 => 400,
        x if x >= 30_000.0 => 300,
        x if x >= 15_000.0 => 150,
        x if x >= 5_000.0 => 50,
        _ => 25,
    }
}

// Storage latency P99: ms -> score (max 600/700)
pub fn storage_latency_score(p99_ms: f64) -> u32 {
    match p99_ms {
        x if x < 0.5 => 700,
        x if x < 1.0 => 550,
        x if x < 2.0 => 400,
        x if x < 5.0 => 250,
        x if x < 10.0 => 150,
        x if x < 25.0 => 75,
        x if x < 50.0 => 30,
        _ => 10,
    }
}

// Metadata ops: ops/sec -> score (max 500)
pub fn metadata_ops_score(ops_per_sec: f64) -> u32 {
    match ops_per_sec {
        x if x >= 5_000.0 => 500,
        x if x >= 3_000.0 => 350,
        x if x >= 1_500.0 => 200,
        x if x >= 500.0 => 100,
        _ => 25,
    }
}

// Sequential read: MB/s -> score (max 500)
pub fn sequential_read_score(mb_per_sec: f64) -> u32 {
    match mb_per_sec {
        x if x >= 3_000.0 => 500,
        x if x >= 2_000.0 => 400,
        x if x >= 1_000.0 => 250,
        x if x >= 500.0 => 150,
        x if x >= 200.0 => 75,
        _ => 25,
    }
}

// Memory latency: ns -> score (max 400)
pub fn memory_latency_score(latency_ns: f64) -> u32 {
    match latency_ns {
        x if x < 70.0 => 400,
        x if x < 90.0 => 300,
        x if x < 120.0 => 200,
        x if x < 150.0 => 100,
        _ => 50,
    }
}

// Process spawn: ms -> score (max 500)
pub fn process_spawn_score(avg_ms: f64) -> u32 {
    match avg_ms {
        x if x < 30.0 => 500,
        x if x < 50.0 => 400,
        x if x < 100.0 => 250,
        x if x < 200.0 => 125,
        x if x < 500.0 => 50,
        _ => 10,
    }
}
```
