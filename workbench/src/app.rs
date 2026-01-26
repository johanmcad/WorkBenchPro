use std::sync::mpsc::Receiver;

use eframe::egui;

use crate::benchmarks::apps::{
    AppLaunchBenchmark, ArchiveOpsBenchmark, CSharpCompileBenchmark, DefenderImpactBenchmark,
    EnvironmentBenchmark, EventLogBenchmark, NetworkBenchmark, PowerShellBenchmark,
    ProcessesBenchmark, RegistryBenchmark, RobocopyBenchmark, ServicesBenchmark, SymlinkBenchmark,
    TaskSchedulerBenchmark, WindowsSearchBenchmark, WmicBenchmark,
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
use crate::cloud::{BrowseFilter, CloudClient, CommunityRun};
use crate::core::{BenchmarkMessage, BenchmarkRunner, RunConfig, SystemInfoCollector};
use crate::export::JsonExporter;
use crate::models::{BenchmarkRun, SystemInfo};
use crate::storage::HistoryStorage;
use crate::ui::views::{
    CommunityAction, CommunityFilters, CommunityView, ComparisonView, HistoryAction, HistoryView,
    HomeView, ResultsAction, ResultsView, RunningView,
};
use crate::ui::Theme;

/// Application state
#[derive(Debug, Clone, PartialEq, Eq)]
enum AppState {
    Home,
    Running,
    Results,
    History,
    Comparison(usize, usize),       // Indices of runs to compare
    ViewingHistoricRun(usize),      // Index of run to view
    CommunityBrowser(Option<usize>), // Browsing community, optional local run index for comparison
    OnlineComparison(usize, String), // Local run index, remote run ID
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

    // Cloud/Community
    cloud_client: CloudClient,
    community_runs: Vec<CommunityRun>,
    community_filters: CommunityFilters,
    community_loading: bool,
    community_error: Option<String>,
    fetched_remote_run: Option<BenchmarkRun>,

    // Upload dialog state
    show_upload_dialog: bool,
    upload_display_name: String,
    upload_in_progress: bool,
    upload_error: Option<String>,
    upload_success: bool,
    upload_run_index: Option<usize>, // Index of run being uploaded (None = last_run)
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
            run_config: RunConfig { machine_name: machine_name.clone() },
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
            // Cloud state
            cloud_client: CloudClient::new(),
            community_runs: Vec::new(),
            community_filters: CommunityFilters::default(),
            community_loading: false,
            community_error: None,
            fetched_remote_run: None,
            // Upload dialog
            show_upload_dialog: false,
            upload_display_name: machine_name,
            upload_in_progress: false,
            upload_error: None,
            upload_success: false,
            upload_run_index: None,
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
            Box::new(RobocopyBenchmark::new()),
            Box::new(WindowsSearchBenchmark::new()),
            Box::new(DefenderImpactBenchmark::new()),
            // Build Performance (CPU + real app benchmarks)
            Box::new(SingleThreadBenchmark::new()),
            Box::new(MultiThreadBenchmark::new()),
            Box::new(MixedWorkloadBenchmark::new()),
            Box::new(SustainedWriteBenchmark::new()),
            Box::new(CSharpCompileBenchmark::new()),
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
                            "{}: {:.2} {}",
                            result.name, result.value, result.unit
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

    fn browse_community(&mut self) {
        self.community_loading = true;
        self.community_error = None;

        let filter = BrowseFilter::new()
            .with_limit(50);

        // Apply filters if set
        let filter = if !self.community_filters.cpu_filter.trim().is_empty() {
            filter.with_cpu(&self.community_filters.cpu_filter)
        } else {
            filter
        };

        let filter = if !self.community_filters.os_filter.trim().is_empty() {
            filter.with_os(&self.community_filters.os_filter)
        } else {
            filter
        };

        let filter = if !self.community_filters.min_memory.trim().is_empty() {
            if let Ok(mem) = self.community_filters.min_memory.parse::<f64>() {
                filter.with_min_memory(mem)
            } else {
                filter
            }
        } else {
            filter
        };

        match self.cloud_client.browse(&filter) {
            Ok(runs) => {
                self.community_runs = runs;
                self.community_loading = false;
            }
            Err(e) => {
                self.community_error = Some(e.to_string());
                self.community_loading = false;
            }
        }
    }

    fn fetch_remote_run(&mut self, id: &str) -> bool {
        match self.cloud_client.fetch(id) {
            Ok(run) => {
                self.fetched_remote_run = Some(run);
                true
            }
            Err(e) => {
                tracing::error!("Failed to fetch remote run: {}", e);
                false
            }
        }
    }

    fn upload_run(&mut self, run: &BenchmarkRun) {
        self.upload_in_progress = true;
        self.upload_error = None;

        match self.cloud_client.upload(run, &self.upload_display_name) {
            Ok(remote_id) => {
                self.upload_in_progress = false;
                self.upload_success = true;

                // Update the run with remote ID
                if let Some(idx) = self.upload_run_index {
                    if let Some(history_run) = self.history_runs.get_mut(idx) {
                        history_run.remote_id = Some(remote_id.clone());
                        history_run.uploaded_at = Some(chrono::Utc::now());
                        // Re-save to persist the remote_id
                        if let Err(e) = self.history_storage.save(history_run) {
                            tracing::error!("Failed to update history with remote ID: {}", e);
                        }
                    }
                } else if let Some(ref mut last) = self.last_run {
                    last.remote_id = Some(remote_id);
                    last.uploaded_at = Some(chrono::Utc::now());
                }

                tracing::info!("Upload successful");
            }
            Err(e) => {
                self.upload_in_progress = false;
                self.upload_error = Some(e.to_string());
            }
        }
    }

    fn reset_upload_dialog(&mut self) {
        self.show_upload_dialog = false;
        self.upload_in_progress = false;
        self.upload_error = None;
        self.upload_success = false;
        self.upload_run_index = None;
        self.upload_display_name = self.run_config.machine_name.clone();
    }
}

impl eframe::App for WorkBenchApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Process any pending messages from the benchmark runner
        self.process_messages();

        // Determine actions needed
        let mut action_start = false;
        let mut action_cancel = false;
        let mut results_action = ResultsAction::None;
        let mut action_history = false;
        let mut history_action = HistoryAction::None;
        let mut comparison_back = false;
        let mut historic_view_back = false;
        let mut community_action = CommunityAction::None;
        let mut online_comparison_back = false;

        egui::CentralPanel::default().show(ctx, |ui| {
            match &self.state {
                AppState::Home => {
                    let (start, history) =
                        HomeView::show_with_history(ui, &self.system_info);
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
                        results_action = ResultsView::show_with_save(ui, run);
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
                AppState::CommunityBrowser(_local_idx) => {
                    community_action = CommunityView::show(
                        ui,
                        &self.community_runs,
                        &mut self.community_filters,
                        self.community_loading,
                        self.community_error.as_deref(),
                    );
                }
                AppState::OnlineComparison(local_idx, _remote_id) => {
                    if let (Some(local_run), Some(remote_run)) = (
                        self.history_runs.get(*local_idx),
                        self.fetched_remote_run.as_ref(),
                    ) {
                        online_comparison_back = ComparisonView::show(ui, local_run, remote_run);
                    }
                }
            }
        });

        // Show upload dialog if active
        let mut upload_should_close = false;
        let mut upload_should_upload = false;

        if self.show_upload_dialog {
            egui::Window::new("Upload to Community")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.set_min_width(300.0);

                    if self.upload_success {
                        // Success state
                        ui.vertical_centered(|ui| {
                            ui.add_space(8.0);
                            ui.label(
                                egui::RichText::new("Upload successful!")
                                    .size(Theme::SIZE_BODY)
                                    .color(Theme::SUCCESS),
                            );
                            ui.add_space(8.0);
                            ui.label(
                                egui::RichText::new("Your results are now visible to the community.")
                                    .size(Theme::SIZE_CAPTION)
                                    .color(Theme::TEXT_SECONDARY),
                            );
                            ui.add_space(12.0);

                            if ui.button("Close").clicked() {
                                upload_should_close = true;
                            }
                        });
                    } else if self.upload_in_progress {
                        // Uploading state
                        ui.vertical_centered(|ui| {
                            ui.add_space(8.0);
                            ui.spinner();
                            ui.label(
                                egui::RichText::new("Uploading...")
                                    .size(Theme::SIZE_BODY)
                                    .color(Theme::TEXT_SECONDARY),
                            );
                            ui.add_space(8.0);
                        });
                    } else {
                        // Input state
                        ui.vertical(|ui| {
                            ui.label(
                                egui::RichText::new("Display name:")
                                    .size(Theme::SIZE_CAPTION)
                                    .color(Theme::TEXT_SECONDARY),
                            );
                            ui.add(
                                egui::TextEdit::singleline(&mut self.upload_display_name)
                                    .desired_width(280.0)
                                    .hint_text("Enter a name for your submission"),
                            );

                            ui.add_space(4.0);
                            ui.label(
                                egui::RichText::new("This name will be visible to everyone")
                                    .size(Theme::SIZE_CAPTION)
                                    .color(Theme::TEXT_SECONDARY)
                                    .italics(),
                            );

                            if let Some(ref err) = self.upload_error {
                                ui.add_space(8.0);
                                ui.label(
                                    egui::RichText::new(format!("Error: {}", err))
                                        .size(Theme::SIZE_CAPTION)
                                        .color(Theme::ERROR),
                                );
                            }

                            ui.add_space(12.0);

                            ui.horizontal(|ui| {
                                if ui.button("Cancel").clicked() {
                                    upload_should_close = true;
                                }

                                ui.add_space(8.0);

                                let upload_btn = egui::Button::new(
                                    egui::RichText::new("Upload")
                                        .color(egui::Color32::WHITE),
                                )
                                .fill(Theme::ACCENT);

                                if ui.add(upload_btn).clicked() && !self.upload_display_name.trim().is_empty() {
                                    upload_should_upload = true;
                                }
                            });
                        });
                    }
                });
        }

        if upload_should_close {
            self.reset_upload_dialog();
        }

        if upload_should_upload {
            // Get the run to upload
            let run_to_upload = if let Some(idx) = self.upload_run_index {
                self.history_runs.get(idx).cloned()
            } else {
                self.last_run.clone()
            };

            if let Some(run) = run_to_upload {
                self.upload_run(&run);
            }
        }

        // Handle actions after UI is done
        if action_start {
            self.start_benchmark();
        }
        if action_cancel {
            self.cancel_benchmark();
        }

        // Handle results actions
        match results_action {
            ResultsAction::None => {}
            ResultsAction::Back => {
                self.state = AppState::Home;
            }
            ResultsAction::Export => {
                self.export_results();
            }
            ResultsAction::History => {
                self.reload_history();
                self.state = AppState::History;
            }
            ResultsAction::CompareOnline => {
                // Find the last_run in history and use that index
                if let Some(ref last) = self.last_run {
                    let idx = self.history_runs.iter().position(|r| r.id == last.id).unwrap_or(0);
                    self.state = AppState::CommunityBrowser(Some(idx));
                    self.browse_community();
                }
            }
            ResultsAction::Upload => {
                self.upload_run_index = None; // Upload last_run
                self.upload_display_name = self.run_config.machine_name.clone();
                self.show_upload_dialog = true;
            }
        }

        if action_history {
            self.reload_history();
            self.state = AppState::History;
        }
        if comparison_back || historic_view_back {
            self.state = AppState::History;
        }
        if online_comparison_back {
            // Go back to community browser with the same local run
            if let AppState::OnlineComparison(local_idx, _) = self.state.clone() {
                self.state = AppState::CommunityBrowser(Some(local_idx));
            }
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
            HistoryAction::CompareOnline(idx) => {
                self.state = AppState::CommunityBrowser(Some(idx));
                self.browse_community();
            }
            HistoryAction::Upload(idx) => {
                self.upload_run_index = Some(idx);
                self.upload_display_name = self.run_config.machine_name.clone();
                self.show_upload_dialog = true;
            }
            HistoryAction::DeleteRun(idx) => {
                self.delete_history_run(idx);
            }
        }

        // Handle community actions
        match community_action {
            CommunityAction::None => {}
            CommunityAction::Back => {
                self.state = AppState::History;
            }
            CommunityAction::SelectForComparison(remote_id) => {
                if let AppState::CommunityBrowser(Some(local_idx)) = self.state {
                    // Fetch the remote run and transition to comparison
                    if self.fetch_remote_run(&remote_id) {
                        self.state = AppState::OnlineComparison(local_idx, remote_id);
                    }
                }
            }
            CommunityAction::Refresh | CommunityAction::FilterChanged => {
                self.browse_community();
            }
        }
    }
}
