use egui::{CollapsingHeader, RichText, Ui};

use crate::models::SystemInfo;
use crate::ui::Theme;

/// Test specification for display
struct TestSpec {
    name: &'static str,
    what_it_does: &'static str,
    how_it_works: &'static str,
    measures: &'static str,
    relevance: &'static str,
}

/// All test specifications organized by category
struct TestSpecs;

impl TestSpecs {
    fn project_operations() -> Vec<TestSpec> {
        vec![
            TestSpec {
                name: "File Enumeration",
                what_it_does: "Tests how fast the system can list files and directories.",
                how_it_works: "Creates 30,000 small text files across 500 directories in temp folder, then recursively enumerates all files using read_dir() 5 times. Measures files processed per second.",
                measures: "Files/second",
                relevance: "Affects IDE project loading, git status, file explorer browsing",
            },
            TestSpec {
                name: "Random Read (4KB)",
                what_it_does: "Tests random access read performance on storage.",
                how_it_works: "Creates a 1GB file filled with random data, then performs 10,000 random 4KB reads at random offsets. Measures latency percentiles (P50, P95, P99).",
                measures: "P99 latency in milliseconds",
                relevance: "Affects loading multiple source files, database queries, IDE responsiveness",
            },
            TestSpec {
                name: "Metadata Operations",
                what_it_does: "Tests file metadata access speed (attributes, size, timestamps).",
                how_it_works: "Creates test files then repeatedly queries file metadata using std::fs::metadata(). Measures operations per second.",
                measures: "Operations/second",
                relevance: "Affects file browsers, backup tools, build systems checking timestamps",
            },
            TestSpec {
                name: "Directory Traversal",
                what_it_does: "Tests combined enumeration and partial file reading.",
                how_it_works: "Creates 30,000 files with content across 500 directories, then traverses all directories reading the first 1KB of each file. Run 5 times.",
                measures: "Files/second",
                relevance: "Simulates grep/ripgrep searching through source code, IDE indexing",
            },
            TestSpec {
                name: "Large File Read",
                what_it_does: "Tests sequential read throughput for large files.",
                how_it_works: "Creates a 2GB file with random data, then reads it sequentially in 1MB chunks. Performed 3 times after a warmup run.",
                measures: "MB/s throughput",
                relevance: "Affects opening large CAD files, video editing, database operations",
            },
            TestSpec {
                name: "Git Operations",
                what_it_does: "Tests real git command performance.",
                how_it_works: "Creates a git repository with 5,000 files across 50 directories, commits them, then benchmarks: git status (10x), git diff (10x), git log (10x), git add (5x).",
                measures: "Average operation time in ms",
                relevance: "Directly measures developer workflow with version control",
            },
            TestSpec {
                name: "Robocopy File Copy",
                what_it_does: "Tests Windows robust file copy performance.",
                how_it_works: "Creates 1,200+ files across 20 directories with varied sizes (1KB-64KB), then runs robocopy /E (copy) and /MIR (mirror) with 4 threads, 5 iterations each.",
                measures: "Average time in seconds",
                relevance: "Measures backup, deployment, and file synchronization speed",
            },
            TestSpec {
                name: "Windows Search",
                what_it_does: "Tests Windows Search indexing service performance.",
                how_it_works: "Queries the Windows Search index for common file patterns using the Windows Search API. Measures query response time.",
                measures: "Query time in ms",
                relevance: "Affects Start menu search, File Explorer search, Outlook search",
            },
            TestSpec {
                name: "Defender Impact",
                what_it_does: "Measures Windows Defender real-time scanning overhead.",
                how_it_works: "Performs file operations and measures the time difference when Defender is actively scanning vs baseline. Reports overhead percentage.",
                measures: "Overhead percentage",
                relevance: "Shows antivirus impact on build times and file operations",
            },
        ]
    }

    fn build_performance() -> Vec<TestSpec> {
        vec![
            TestSpec {
                name: "Single-Thread CPU",
                what_it_does: "Tests single-core computational performance.",
                how_it_works: "Generates 256MB of random data, then compresses it using LZ4 algorithm on a single CPU core. Repeated 5 times.",
                measures: "MB/s throughput",
                relevance: "Affects single-threaded build steps, script execution, startup time",
            },
            TestSpec {
                name: "Multi-Thread CPU",
                what_it_does: "Tests parallel computational performance across all cores.",
                how_it_works: "Generates data chunks for each CPU thread, then compresses all chunks in parallel using LZ4 with rayon. Measures aggregate throughput.",
                measures: "MB/s throughput",
                relevance: "Affects parallel compilation, video encoding, data processing",
            },
            TestSpec {
                name: "Mixed Workload",
                what_it_does: "Tests realistic mixed CPU operations.",
                how_it_works: "Runs compression, hashing (SHA-like), and sorting operations concurrently across multiple threads. Simulates real-world mixed workloads.",
                measures: "MB/s throughput",
                relevance: "Represents typical development workloads with varied operations",
            },
            TestSpec {
                name: "Sustained Write",
                what_it_does: "Tests continuous write performance with periodic sync.",
                how_it_works: "Writes 4GB of data in 4MB chunks, calling fsync every 256MB to simulate realistic build output. Performed 2 times.",
                measures: "MB/s throughput",
                relevance: "Affects build artifact generation, log writing, database commits",
            },
            TestSpec {
                name: "Native Compiler",
                what_it_does: "Tests Windows native C# compilation performance.",
                how_it_works: "Generates 5 C# source files with classes, generics, and 30+ math functions, then compiles with csc.exe /optimize+. Repeated 5 times.",
                measures: "Average compile time in seconds",
                relevance: "Measures build performance using Windows built-in compiler",
            },
            TestSpec {
                name: "Archive Operations",
                what_it_does: "Tests archive compression and extraction speed.",
                how_it_works: "Creates 1,500 text files (~50MB total) across 30 directories, then compresses with tar -czf and extracts with tar -xzf. Repeated 5 times each.",
                measures: "Total time in seconds",
                relevance: "Affects npm install, artifact packaging, backup operations",
            },
            TestSpec {
                name: "PowerShell Scripts",
                what_it_does: "Tests PowerShell script execution performance.",
                how_it_works: "Executes 4 different scripts: compute (loops/math), file operations (create/read/delete), object manipulation (1000 PSObjects), and string processing. 5 runs each.",
                measures: "Average script time in ms",
                relevance: "Affects build scripts, automation, deployment pipelines",
            },
        ]
    }

    fn responsiveness() -> Vec<TestSpec> {
        vec![
            TestSpec {
                name: "Storage Latency",
                what_it_does: "Measures storage I/O latency distribution.",
                how_it_works: "Creates 1GB test file, performs 10,000 random 4KB reads measuring individual operation times. Reports P50, P75, P90, P95, P99, P99.9 percentiles.",
                measures: "P99 latency in ms",
                relevance: "Shows SSD/HDD responsiveness, affects perceived system snappiness",
            },
            TestSpec {
                name: "Process Spawn",
                what_it_does: "Tests process creation overhead.",
                how_it_works: "Spawns 'cmd /c echo test' (Windows) or 'sh -c echo test' (Linux) 100 times, measuring time from spawn() to process completion.",
                measures: "Average spawn time in ms",
                relevance: "Affects build tools that spawn many processes (make, npm, cargo)",
            },
            TestSpec {
                name: "Thread Wake",
                what_it_does: "Tests thread scheduler responsiveness.",
                how_it_works: "Creates worker threads that sleep, then measures time to wake them using condition variables. Captures latency distribution.",
                measures: "P99 wake latency in μs",
                relevance: "Affects async runtime performance, UI responsiveness, server latency",
            },
            TestSpec {
                name: "Memory Latency",
                what_it_does: "Tests memory subsystem access latency.",
                how_it_works: "Creates a large array with pointer-chasing pattern (each element points to another random element), then follows the chain measuring access time.",
                measures: "Average latency in nanoseconds",
                relevance: "Affects cache-unfriendly workloads, large data structure traversal",
            },
            TestSpec {
                name: "Memory Bandwidth",
                what_it_does: "Tests memory throughput across all cores.",
                how_it_works: "Allocates large buffers per thread (total ~1GB), performs memcpy operations in parallel using all CPU cores. Measures aggregate bandwidth.",
                measures: "GB/s throughput",
                relevance: "Affects data processing, video editing, scientific computing",
            },
        ]
    }

    fn system_tools() -> Vec<TestSpec> {
        vec![
            TestSpec {
                name: "Registry Operations",
                what_it_does: "Tests Windows Registry access speed.",
                how_it_works: "Opens registry keys under HKLM and HKCU, reads and writes test values, enumerates subkeys. Measures operations per second.",
                measures: "Operations/second",
                relevance: "Affects app startup (reading settings), installers, system tools",
            },
            TestSpec {
                name: "Event Log",
                what_it_does: "Tests Windows Event Log query performance.",
                how_it_works: "Queries the Application, System, and Security event logs using Windows Event Log API. Measures time to retrieve recent entries.",
                measures: "Query time in ms",
                relevance: "Affects monitoring tools, debugging, security auditing",
            },
            TestSpec {
                name: "Task Scheduler",
                what_it_does: "Tests Task Scheduler API performance.",
                how_it_works: "Queries the Windows Task Scheduler for all scheduled tasks, enumerates task folders and retrieves task properties.",
                measures: "Query time in ms",
                relevance: "Affects system administration tools, backup schedulers",
            },
            TestSpec {
                name: "Application Launch",
                what_it_does: "Tests built-in Windows application startup time.",
                how_it_works: "Launches Notepad, WordPad, Calculator, MSPaint, and cmd.exe 5 times each. Measures time until process is running, then terminates it.",
                measures: "Average launch time in ms",
                relevance: "Indicates overall system responsiveness for starting applications",
            },
            TestSpec {
                name: "Services Query",
                what_it_does: "Tests Windows Service Manager performance.",
                how_it_works: "Enumerates all Windows services using Service Control Manager API, retrieves service status and configuration.",
                measures: "Query time in ms",
                relevance: "Affects services.msc, system administration, monitoring tools",
            },
            TestSpec {
                name: "Network Info",
                what_it_does: "Tests network configuration query speed.",
                how_it_works: "Queries network adapters, IP configuration, routing tables, and DNS settings using Windows networking APIs.",
                measures: "Query time in ms",
                relevance: "Affects network troubleshooting tools, VPN clients, system info",
            },
            TestSpec {
                name: "WMI Query",
                what_it_does: "Tests Windows Management Instrumentation performance.",
                how_it_works: "Executes WMI queries via wmic.exe for system information: OS details, CPU info, disk drives, memory. Measures query response time.",
                measures: "Query time in ms",
                relevance: "Affects system monitoring, inventory tools, PowerShell scripts",
            },
            TestSpec {
                name: "Process List",
                what_it_does: "Tests process enumeration speed.",
                how_it_works: "Enumerates all running processes using Windows API or tasklist command, retrieves process names, PIDs, and memory usage.",
                measures: "Query time in ms",
                relevance: "Affects Task Manager, process monitors, debugging tools",
            },
            TestSpec {
                name: "Symlink Operations",
                what_it_does: "Tests symbolic link creation and resolution.",
                how_it_works: "Creates directory and file symbolic links, then resolves them to their targets. Measures operations per second.",
                measures: "Operations/second",
                relevance: "Affects npm/pnpm (uses symlinks), development workflows, junctions",
            },
            TestSpec {
                name: "Environment Variables",
                what_it_does: "Tests environment variable access speed.",
                how_it_works: "Reads and writes environment variables using Windows API, enumerates all variables. Measures operations per second.",
                measures: "Operations/second",
                relevance: "Affects process startup, build scripts, configuration loading",
            },
        ]
    }
}

/// Home View matching 05-ui-design.md spec:
/// - App title + tagline
/// - System info card
/// - Test config checkboxes
/// - Run button (large, centered)
/// - Previous results dropdown
pub struct HomeView;

impl HomeView {
    /// Returns (run_clicked, history_clicked)
    pub fn show_with_history(
        ui: &mut Ui,
        system_info: &SystemInfo,
    ) -> (bool, bool) {
        let mut run_clicked = false;
        let mut history_clicked = false;

        egui::ScrollArea::vertical().show(ui, |ui| {
            // Compact header - left aligned
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("WorkBench")
                        .size(Theme::SIZE_SECTION)
                        .strong()
                        .color(Theme::ACCENT),
                );
                ui.label(
                    RichText::new("— Developer Workstation Benchmark")
                        .size(Theme::SIZE_BODY)
                        .color(Theme::TEXT_SECONDARY),
                );
            });

            ui.add_space(6.0);

            // System Info Card - compact
            egui::Frame::none()
                .fill(Theme::BG_CARD)
                .stroke(egui::Stroke::new(1.0, Theme::BORDER))
                .rounding(Theme::CARD_ROUNDING)
                .inner_margin(8.0)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("System")
                                .size(Theme::SIZE_BODY)
                                .strong()
                                .color(Theme::TEXT_PRIMARY),
                        );
                        ui.separator();
                        egui::Grid::new("system_info_grid")
                            .num_columns(6)
                            .spacing([8.0, 2.0])
                            .show(ui, |ui| {
                                ui.label(RichText::new("Host:").size(Theme::SIZE_CAPTION).color(Theme::TEXT_SECONDARY));
                                ui.label(RichText::new(&system_info.hostname).size(Theme::SIZE_CAPTION));
                                ui.label(RichText::new("CPU:").size(Theme::SIZE_CAPTION).color(Theme::TEXT_SECONDARY));
                                ui.label(RichText::new(&system_info.cpu.name).size(Theme::SIZE_CAPTION));
                                ui.label(RichText::new("Cores:").size(Theme::SIZE_CAPTION).color(Theme::TEXT_SECONDARY));
                                ui.label(RichText::new(format!("{}/{}", system_info.cpu.cores, system_info.cpu.threads)).size(Theme::SIZE_CAPTION));
                            });
                    });
                    ui.horizontal(|ui| {
                        ui.add_space(48.0); // Align with content above
                        egui::Grid::new("system_info_grid2")
                            .num_columns(6)
                            .spacing([8.0, 2.0])
                            .show(ui, |ui| {
                                ui.label(RichText::new("RAM:").size(Theme::SIZE_CAPTION).color(Theme::TEXT_SECONDARY));
                                ui.label(RichText::new(format!("{:.0}GB", system_info.memory.total_gb())).size(Theme::SIZE_CAPTION));
                                ui.label(RichText::new("OS:").size(Theme::SIZE_CAPTION).color(Theme::TEXT_SECONDARY));
                                ui.label(RichText::new(format!("{} {}", system_info.os.name, system_info.os.version)).size(Theme::SIZE_CAPTION));
                                if !system_info.storage.is_empty() {
                                    let storage = &system_info.storage[0];
                                    ui.label(RichText::new("Disk:").size(Theme::SIZE_CAPTION).color(Theme::TEXT_SECONDARY));
                                    ui.label(RichText::new(format!("{} {:.0}GB", storage.device_type.label(), storage.capacity_gb())).size(Theme::SIZE_CAPTION));
                                }
                            });
                    });
                });

            ui.add_space(6.0);

            // Test Specifications Card - compact
            egui::Frame::none()
                .fill(Theme::BG_CARD)
                .stroke(egui::Stroke::new(1.0, Theme::BORDER))
                .rounding(Theme::CARD_ROUNDING)
                .inner_margin(8.0)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("Test Details")
                                .size(Theme::SIZE_BODY)
                                .strong()
                                .color(Theme::TEXT_PRIMARY),
                        );
                        ui.separator();
                        ui.label(
                            RichText::new("31 tests")
                                .size(Theme::SIZE_CAPTION)
                                .color(Theme::TEXT_SECONDARY),
                        );
                    });

                    ui.add_space(4.0);

                    // Project Operations
                    CollapsingHeader::new(
                        RichText::new("Project Operations (9)")
                            .size(Theme::SIZE_CAPTION)
                            .color(Theme::ACCENT),
                    )
                    .default_open(false)
                    .show(ui, |ui| {
                        Self::show_test_list(ui, "project_ops", &TestSpecs::project_operations());
                    });

                    // Build Performance
                    CollapsingHeader::new(
                        RichText::new("Build Performance (7)")
                            .size(Theme::SIZE_CAPTION)
                            .color(Theme::ACCENT),
                    )
                    .default_open(false)
                    .show(ui, |ui| {
                        Self::show_test_list(ui, "build_perf", &TestSpecs::build_performance());
                    });

                    // Responsiveness
                    CollapsingHeader::new(
                        RichText::new("Responsiveness (5)")
                            .size(Theme::SIZE_CAPTION)
                            .color(Theme::ACCENT),
                    )
                    .default_open(false)
                    .show(ui, |ui| {
                        Self::show_test_list(ui, "responsiveness", &TestSpecs::responsiveness());
                    });

                    // System Tools
                    CollapsingHeader::new(
                        RichText::new("Windows System Tools (10)")
                            .size(Theme::SIZE_CAPTION)
                            .color(Theme::ACCENT),
                    )
                    .default_open(false)
                    .show(ui, |ui| {
                        Self::show_test_list(ui, "system_tools", &TestSpecs::system_tools());
                    });
                });

            ui.add_space(6.0);

            // Buttons - compact
            ui.horizontal(|ui| {
                let run_button = egui::Button::new(
                    RichText::new("Run Benchmark")
                        .size(Theme::SIZE_BODY)
                        .strong()
                        .color(egui::Color32::WHITE),
                )
                .min_size(egui::vec2(120.0, 28.0))
                .fill(Theme::ACCENT)
                .rounding(4.0);

                if ui.add(run_button).clicked() {
                    run_clicked = true;
                }

                ui.add_space(4.0);

                let history_button = egui::Button::new(
                    RichText::new("History").size(Theme::SIZE_CAPTION),
                )
                .min_size(egui::vec2(70.0, 28.0))
                .rounding(4.0);

                if ui.add(history_button).clicked() {
                    history_clicked = true;
                }
            });

            ui.add_space(4.0);
        });

        (run_clicked, history_clicked)
    }

    fn show_test_list(ui: &mut Ui, id: &str, tests: &[TestSpec]) {
        egui::Grid::new(format!("test_list_{}", id))
            .num_columns(3)
            .spacing([12.0, 2.0])
            .striped(true)
            .show(ui, |ui| {
                // Header
                ui.label(RichText::new("Test").size(Theme::SIZE_CAPTION).strong().color(Theme::TEXT_SECONDARY));
                ui.label(RichText::new("Measures").size(Theme::SIZE_CAPTION).strong().color(Theme::TEXT_SECONDARY));
                ui.label(RichText::new("Relevance").size(Theme::SIZE_CAPTION).strong().color(Theme::TEXT_SECONDARY));
                ui.end_row();

                for test in tests.iter() {
                    ui.label(RichText::new(test.name).size(Theme::SIZE_CAPTION).color(Theme::TEXT_PRIMARY));
                    ui.label(RichText::new(test.measures).size(Theme::SIZE_CAPTION).color(Theme::ACCENT));
                    ui.label(RichText::new(test.relevance).size(Theme::SIZE_CAPTION).color(Theme::TEXT_SECONDARY));
                    ui.end_row();
                }
            });
    }
}
