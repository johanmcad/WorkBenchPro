# WorkBench-Pro

A developer workstation benchmarking tool for Windows, built with Rust.

WorkBench-Pro measures real-world developer workstation performance through benchmarks that simulate actual development workflows - not synthetic tests.

## Features

- **CPU Benchmarks**: Single-thread, multi-thread, mixed workload, sustained performance
- **Disk Benchmarks**: File enumeration, large file reads, random I/O, metadata operations, directory traversal
- **Memory Benchmarks**: Bandwidth and latency measurements
- **Latency Benchmarks**: Process spawn time, storage latency, thread wake latency
- **Windows-Specific Benchmarks**: PowerShell, Registry, Services, Event Log, Task Scheduler, Windows Search, Defender impact, and more
- **Community Comparison**: Upload and compare your results with others
- **History Tracking**: Track performance over time
- **Export**: Save results as JSON or HTML reports

## Screenshots

<!-- Add screenshots here -->

## Installation

### Pre-built Binaries

Download the latest release from the [Releases](https://github.com/johanmcad/workbench-pro/releases) page.

### Building from Source

#### Prerequisites

- [Rust](https://rustup.rs/) (1.70 or later)
- Windows 10/11 (primary target)

#### Build

```bash
# Clone the repository
git clone https://github.com/johanmcad/workbench-pro.git
cd workbench-pro

# Build release binary
cargo build --release

# The binary will be at target/release/workbench-pro.exe
```

## Antivirus Notice

Some antivirus software may flag WorkBench-Pro as suspicious. This is a **false positive** caused by the benchmarking behaviors:

- Collecting system hardware information
- Spawning multiple processes (PowerShell, cmd, git, etc.)
- Creating and deleting thousands of test files
- Querying Windows services and registry
- Measuring Windows Defender performance impact

If your antivirus blocks the application, you can:
1. Add an exception for `workbench-pro.exe`
2. Build from source and verify the code yourself
3. Report the false positive to your AV vendor

## Usage

Simply run the executable:

```bash
./workbench-pro.exe
```

The GUI will guide you through selecting and running benchmarks.

## How It Works

WorkBench-Pro focuses on real-world developer operations:

| Category | What It Measures |
|----------|-----------------|
| CPU | Compile-like workloads, parallel processing, sustained performance |
| Disk | File operations developers do daily (git, npm, file browsing) |
| Memory | RAM speed for large codebases and build processes |
| Latency | Responsiveness of common operations |
| Windows | OS-specific operations that affect developer experience |

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [egui](https://github.com/emilk/egui) for the GUI
- Uses [sysinfo](https://github.com/GuillaumeGomez/sysinfo) for system information
