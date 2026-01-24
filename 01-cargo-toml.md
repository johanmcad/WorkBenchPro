# Cargo.toml

```toml
[package]
name = "workbench"
version = "0.1.0"
edition = "2021"
description = "Workstation Benchmark - Developer productivity measurement"
license = "MIT"

[dependencies]
# UI
eframe = "0.29"
egui = "0.29"
egui_plot = "0.29"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# System info
sysinfo = "0.32"
raw-cpuid = "11"

# Utilities
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.0", features = ["v4", "serde"] }
thiserror = "2.0"
anyhow = "1.0"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# Parallelism
rayon = "1.10"

# Random
rand = "0.8"

# Compression (CPU benchmark)
lz4_flex = "0.11"

# Image (icon)
image = { version = "0.25", default-features = false, features = ["png"] }

# Windows
[target.'cfg(windows)'.dependencies]
windows = { version = "0.58", features = [
    "Win32_Foundation",
    "Win32_System_Performance",
    "Win32_System_SystemInformation",
    "Win32_Storage_FileSystem",
    "Win32_System_Threading",
    "Win32_Graphics_Dxgi",
]}

[build-dependencies]
winres = "0.1"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
panic = "abort"
```
