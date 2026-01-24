/// File enumeration: files/sec -> score (max 500)
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

/// Storage latency P99: ms -> score (max 700)
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

/// Metadata ops: ops/sec -> score (max 500)
pub fn metadata_ops_score(ops_per_sec: f64) -> u32 {
    match ops_per_sec {
        x if x >= 5_000.0 => 500,
        x if x >= 3_000.0 => 350,
        x if x >= 1_500.0 => 200,
        x if x >= 500.0 => 100,
        _ => 25,
    }
}

/// Traversal: files/sec -> score (max 400)
pub fn traversal_score(files_per_sec: f64) -> u32 {
    match files_per_sec {
        x if x >= 20_000.0 => 400,
        x if x >= 10_000.0 => 300,
        x if x >= 5_000.0 => 150,
        x if x >= 1_000.0 => 75,
        _ => 25,
    }
}

/// Sequential read: MB/s -> score (max 500)
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

/// Single-thread compute: MB/s -> score (max 600)
pub fn single_thread_score(mb_per_sec: f64) -> u32 {
    match mb_per_sec {
        x if x >= 500.0 => 600,
        x if x >= 350.0 => 450,
        x if x >= 200.0 => 300,
        x if x >= 100.0 => 150,
        _ => 50,
    }
}

/// Multi-thread compute: MB/s -> score (max 600)
pub fn multi_thread_score(mb_per_sec: f64) -> u32 {
    match mb_per_sec {
        x if x >= 4_000.0 => 600,
        x if x >= 2_500.0 => 450,
        x if x >= 1_500.0 => 300,
        x if x >= 800.0 => 150,
        _ => 50,
    }
}

/// Mixed workload: MB/s -> score (max 700)
pub fn mixed_workload_score(mb_per_sec: f64) -> u32 {
    match mb_per_sec {
        x if x >= 1_000.0 => 700,
        x if x >= 600.0 => 500,
        x if x >= 300.0 => 300,
        x if x >= 150.0 => 150,
        _ => 50,
    }
}

/// Sustained write: MB/s -> score (max 600)
pub fn sustained_write_score(mb_per_sec: f64) -> u32 {
    match mb_per_sec {
        x if x >= 2_500.0 => 600,
        x if x >= 1_500.0 => 450,
        x if x >= 800.0 => 300,
        x if x >= 400.0 => 150,
        x if x >= 200.0 => 75,
        _ => 50,
    }
}

/// Memory latency: ns -> score (max 400)
pub fn memory_latency_score(latency_ns: f64) -> u32 {
    match latency_ns {
        x if x < 70.0 => 400,
        x if x < 90.0 => 300,
        x if x < 120.0 => 200,
        x if x < 150.0 => 100,
        _ => 50,
    }
}

/// Process spawn: ms -> score (max 500)
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

/// Thread wake latency: us -> score (max 400)
pub fn thread_wake_score(latency_us: f64) -> u32 {
    match latency_us {
        x if x < 50.0 => 400,
        x if x < 100.0 => 300,
        x if x < 200.0 => 200,
        x if x < 500.0 => 100,
        _ => 50,
    }
}

/// Memory bandwidth: GB/s -> score (max 500)
pub fn memory_bandwidth_score(gb_per_sec: f64) -> u32 {
    match gb_per_sec {
        x if x >= 50.0 => 500,
        x if x >= 40.0 => 400,
        x if x >= 30.0 => 300,
        x if x >= 20.0 => 200,
        x if x >= 15.0 => 100,
        _ => 50,
    }
}

/// GPU detection score (max 300)
pub fn gpu_score(is_dedicated: bool, is_integrated: bool) -> u32 {
    if is_dedicated {
        300
    } else if is_integrated {
        200
    } else {
        50
    }
}
