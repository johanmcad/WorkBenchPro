use std::sync::mpsc::Receiver;

use eframe::egui;

use crate::benchmarks::apps::{
    AppLaunchBenchmark, ArchiveOpsBenchmark, CargoBuildBenchmark, DefenderImpactBenchmark,
    EnvironmentBenchmark, EventLogBenchmark, GitOperationsBenchmark, NetworkBenchmark,
    PowerShellBenchmark, ProcessesBenchmark, RegistryBenchmark, RobocopyBenchmark,
    ServicesBenchmark, SymlinkBenchmark, TaskSchedulerBenchmark, WindowsSearchBenchmark,
    WmicBenchmark,
};
use crate::benchmarks::cpu::{
    MixedWorkloadBenchmark, MultiThreadBenchmark, SingleThreadBenchmark, SustainedWriteBenchmark,
};
use crate::benchmarks::disk::{
    FileEnumerationBenchmark, LargeFileReadBenchmark, MetadataOpsBenchmark, RandomReadBenchmark,
    TraversalBenchmark,
};
use crate::benchmarks::latency::{
    ProcessSpawnBenchmark, StorageLatencyBenchmark, ThreadWakeBenchmark,
};
use crate::benchmarks::memory::{MemoryBandwidthBenchmark, MemoryLatencyBenchmark};
use crate::benchmarks::Benchmark;
use crate::core::{BenchmarkMessage, BenchmarkRunner, RunConfig, SystemInfoCollector};
use crate::export::JsonExporter;
use crate::models::{BenchmarkRun, SystemInfo};
use crate::storage::HistoryStorage;
use crate::ui::views::{
    ComparisonView, HistoryAction, HistoryView, HomeView, ResultsView, RunningView,
};
use crate::ui::Theme;

/// Application state
#[derive(Debug, Clone, PartialEq, Eq)]
enum AppState {
    Home,
    Running,
    Results,
    History,
    Comparison(usize, usize), // Indices of runs to compare
    ViewingHistoricRun(usize), // Index of run to view
}

/// Main application
pub struct WorkBenchApp {
    state: AppState,
    system_info: SystemInfo,
    run_config: RunConfig,
    runner: BenchmarkRunner,
    receiver: Option<Receiver<BenchmarkMessage>>,

    // Running state
    overall_progress: f32,
    current_test: String,
    current_message: String,
    completed_tests: Vec<String>,

    // Results
    last_run: Option<BenchmarkRun>,

    // History
    history_storage: HistoryStorage,
    history_runs: Vec<BenchmarkRun>,
    history_selected: Vec<bool>,
}

impl WorkBenchApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Apply theme
        Theme::apply(&cc.egui_ctx);

        // Collect system info
        let system_info = SystemInfoCollector::collect();
        let machine_name = system_info.hostname.clone();

        // Load history
        let history_storage = HistoryStorage::new();
        let history_runs = history_storage.load_all().unwrap_or_default();
        let history_selected = vec![false; history_runs.len()];

        Self {
            state: AppState::Home,
            system_info,
            run_config: RunConfig {
                machine_name,
                ..Default::default()
            },
            runner: BenchmarkRunner::new(),
            receiver: None,
            overall_progress: 0.0,
            current_test: String::new(),
            current_message: String::new(),
            completed_tests: Vec::new(),
            last_run: None,
            history_storage,
            history_runs,
            history_selected,
        }
    }

    fn start_benchmark(&mut self) {
        // Create benchmark instances
        let benchmarks: Vec<Box<dyn Benchmark>> = vec![
            // Project Operations (disk + file operations)
            Box::new(FileEnumerationBenchmark::new()),
            Box::new(RandomReadBenchmark::new()),
            Box::new(MetadataOpsBenchmark::new()),
            Box::new(TraversalBenchmark::new()),
            Box::new(LargeFileReadBenchmark::new()),
            Box::new(GitOperationsBenchmark::new()),
            Box::new(RobocopyBenchmark::new()),
            Box::new(WindowsSearchBenchmark::new()),
            Box::new(DefenderImpactBenchmark::new()),
            // Build Performance (CPU + real app benchmarks)
            Box::new(SingleThreadBenchmark::new()),
            Box::new(MultiThreadBenchmark::new()),
            Box::new(MixedWorkloadBenchmark::new()),
            Box::new(SustainedWriteBenchmark::new()),
            Box::new(CargoBuildBenchmark::new()),
            Box::new(ArchiveOpsBenchmark::new()),
            Box::new(PowerShellBenchmark::new()),
            // Responsiveness (latency + memory benchmarks)
            Box::new(StorageLatencyBenchmark::new()),
            Box::new(ProcessSpawnBenchmark::new()),
            Box::new(ThreadWakeBenchmark::new()),
            Box::new(MemoryLatencyBenchmark::new()),
            Box::new(MemoryBandwidthBenchmark::new()),
            // Windows System Tools
            Box::new(RegistryBenchmark::new()),
            Box::new(EventLogBenchmark::new()),
            Box::new(TaskSchedulerBenchmark::new()),
            Box::new(AppLaunchBenchmark::new()),
            Box::new(ServicesBenchmark::new()),
            Box::new(NetworkBenchmark::new()),
            Box::new(WmicBenchmark::new()),
            Box::new(ProcessesBenchmark::new()),
            Box::new(SymlinkBenchmark::new()),
            Box::new(EnvironmentBenchmark::new()),
        ];

        // Reset running state
        self.overall_progress = 0.0;
        self.current_test = String::new();
        self.current_message = "Starting...".to_string();
        self.completed_tests.clear();

        // Start runner
        let receiver = self.runner.start(benchmarks, self.run_config.clone());
        self.receiver = Some(receiver);
        self.state = AppState::Running;
    }

    fn cancel_benchmark(&mut self) {
        self.runner.cancel();
        self.state = AppState::Home;
        self.receiver = None;
    }

    fn process_messages(&mut self) {
        let receiver = self.receiver.take();

        if let Some(rx) = receiver {
            let mut should_keep_receiver = true;

            while let Ok(msg) = rx.try_recv() {
                match msg {
                    BenchmarkMessage::Progress {
                        benchmark_id: _,
                        progress,
                        message,
                    } => {
                        self.overall_progress = progress;
                        self.current_message = message;
                    }
                    BenchmarkMessage::TestComplete { result } => {
                        self.current_test = result.name.clone();
                        self.completed_tests.push(format!(
                            "{}: {:.2} {} ({}/{})",
                            result.name, result.value, result.unit, result.score, result.max_score
                        ));
                    }
                    BenchmarkMessage::AllComplete { run } => {
                        // Save to history
                        if let Err(e) = self.history_storage.save(&run) {
                            tracing::error!("Failed to save to history: {}", e);
                        }

                        self.last_run = Some(*run);
                        self.state = AppState::Results;
                        should_keep_receiver = false;

                        // Reload history
                        self.reload_history();
                    }
                    BenchmarkMessage::Error { error } => {
                        tracing::error!("Benchmark error: {}", error);
                    }
                    BenchmarkMessage::Cancelled => {
                        self.state = AppState::Home;
                        should_keep_receiver = false;
                    }
                }
            }

            if should_keep_receiver {
                self.receiver = Some(rx);
            }
        }
    }

    fn reload_history(&mut self) {
        self.history_runs = self.history_storage.load_all().unwrap_or_default();
        self.history_selected = vec![false; self.history_runs.len()];
    }

    fn export_results(&self) {
        if let Some(run) = &self.last_run {
            let path = std::env::temp_dir().join(format!(
                "workbench_results_{}.json",
                run.timestamp.format("%Y%m%d_%H%M%S")
            ));

            match JsonExporter::export(run, &path) {
                Ok(()) => {
                    tracing::info!("Results exported to {:?}", path);
                }
                Err(e) => {
                    tracing::error!("Failed to export results: {}", e);
                }
            }
        }
    }

    fn delete_history_run(&mut self, idx: usize) {
        if idx < self.history_runs.len() {
            let run = &self.history_runs[idx];
            if let Err(e) = self.history_storage.delete(run) {
                tracing::error!("Failed to delete run: {}", e);
            }
            self.reload_history();
        }
    }
}

impl eframe::App for WorkBenchApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Process any pending messages from the benchmark runner
        self.process_messages();

        // Determine actions needed
        let mut action_start = false;
        let mut action_cancel = false;
        let mut action_back = false;
        let mut action_export = false;
        let mut action_history = false;
        let mut action_save = false;
        let mut history_action = HistoryAction::None;
        let mut comparison_back = false;
        let mut historic_view_back = false;

        egui::CentralPanel::default().show(ctx, |ui| {
            match &self.state {
                AppState::Home => {
                    let (start, history) =
                        HomeView::show_with_history(ui, &self.system_info, &mut self.run_config);
                    action_start = start;
                    action_history = history;
                }
                AppState::Running => {
                    action_cancel = RunningView::show(
                        ui,
                        self.overall_progress,
                        &self.current_test,
                        &self.current_message,
                        &self.completed_tests,
                    );
                }
                AppState::Results => {
                    if let Some(run) = &self.last_run {
                        let (back, export, save) = ResultsView::show_with_save(ui, run);
                        action_back = back;
                        action_export = export;
                        action_save = save;
                    }
                }
                AppState::History => {
                    history_action =
                        HistoryView::show(ui, &self.history_runs, &mut self.history_selected);
                }
                AppState::Comparison(idx_a, idx_b) => {
                    if let (Some(run_a), Some(run_b)) = (
                        self.history_runs.get(*idx_a),
                        self.history_runs.get(*idx_b),
                    ) {
                        comparison_back = ComparisonView::show(ui, run_a, run_b);
                    }
                }
                AppState::ViewingHistoricRun(idx) => {
                    if let Some(run) = self.history_runs.get(*idx) {
                        let (back, _export) = ResultsView::show(ui, run);
                        historic_view_back = back;
                    }
                }
            }
        });

        // Handle actions after UI is done
        if action_start {
            self.start_benchmark();
        }
        if action_cancel {
            self.cancel_benchmark();
        }
        if action_back {
            self.state = AppState::Home;
        }
        if action_export {
            self.export_results();
        }
        if action_history {
            self.reload_history();
            self.state = AppState::History;
        }
        if action_save {
            // Already saved automatically when benchmark completes
            tracing::info!("Results saved to history");
        }
        if comparison_back || historic_view_back {
            self.state = AppState::History;
        }

        // Handle history actions
        match history_action {
            HistoryAction::None => {}
            HistoryAction::Back => {
                self.state = AppState::Home;
            }
            HistoryAction::ViewRun(idx) => {
                self.state = AppState::ViewingHistoricRun(idx);
            }
            HistoryAction::CompareRuns(idx_a, idx_b) => {
                self.state = AppState::Comparison(idx_a, idx_b);
            }
            HistoryAction::DeleteRun(idx) => {
                self.delete_history_run(idx);
            }
        }
    }
}
