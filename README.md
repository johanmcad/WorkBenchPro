# WorkBench

Lightweight developer workstation benchmark tool built with Rust and egui.

## Goals

- Lightweight: ~5-10 MB binary, instant startup, low memory
- Accurate: Precise measurements, especially latency distributions
- Professional: Clean UI, management-ready reports
- Objective: Let numbers speak, no bias labels
- Comparable: Easy comparison between machines

## Tech Stack

- Language: Rust 2021 edition
- UI: egui + eframe
- Charts: egui_plot
- Serialization: serde + serde_json
- System info: sysinfo, raw-cpuid
- Platform: Windows primary

## Project Structure

```
workbench/
├── Cargo.toml
├── build.rs
├── src/
│   ├── main.rs
│   ├── app.rs
│   ├── core/
│   │   ├── mod.rs
│   │   ├── system_info.rs
│   │   ├── timer.rs
│   │   └── runner.rs
│   ├── benchmarks/
│   │   ├── mod.rs
│   │   ├── traits.rs
│   │   ├── disk/
│   │   ├── cpu/
│   │   ├── memory/
│   │   ├── latency/
│   │   └── graphics/
│   ├── scoring/
│   │   ├── mod.rs
│   │   ├── calculator.rs
│   │   └── thresholds.rs
│   ├── models/
│   │   ├── mod.rs
│   │   ├── system_info.rs
│   │   ├── results.rs
│   │   └── report.rs
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── theme.rs
│   │   ├── widgets/
│   │   └── views/
│   └── export/
│       ├── mod.rs
│       ├── json.rs
│       └── html.rs
└── assets/
    ├── fonts/
    └── icon.ico
```

## Development Phases

1. Foundation - Project setup, system info, timer, basic UI
2. Core Benchmarks - Disk and latency tests
3. CPU/Memory + UI - Remaining benchmarks, results view
4. Scoring + Export - Scoring system, JSON, comparison
5. Polish - HTML reports, graphics tests (optional), release

## Plan Files

- README.md (this file)
- 01-cargo-toml.md - Dependencies
- 02-models.md - Data structures
- 03-benchmarks.md - Test specifications
- 04-scoring.md - Scoring system
- 05-ui-design.md - Colors, components
- 06-phases.md - Development tasks
