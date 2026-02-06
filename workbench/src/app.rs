use std::sync::mpsc::{self, Receiver};

use eframe::egui;
use sha2::{Sha256, Digest};

use crate::benchmarks::apps::{
    AppLaunchBenchmark, ArchiveOpsBenchmark, DefenderImpactBenchmark,
    EnvironmentBenchmark, EventLogBenchmark, PowerShellBenchmark,
    ProcessesBenchmark, RegistryBenchmark, RobocopyBenchmark, ServicesBenchmark, SymlinkBenchmark,
    TaskSchedulerBenchmark, WindowsCompressionBenchmark, WindowsSearchBenchmark, WmicBenchmark,
};
use crate::benchmarks::cpu::{
    MixedWorkloadBenchmark, MultiThreadBenchmark, SingleThreadBenchmark, SustainedWriteBenchmark,
};
use crate::benchmarks::disk::{
    FileEnumerationBenchmark, LargeFileReadBenchmark, LargeFileReadLiteBenchmark,
    MetadataOpsBenchmark, RandomReadBenchmark, RandomReadLiteBenchmark, TraversalBenchmark,
};
use crate::benchmarks::latency::{
    ProcessSpawnBenchmark, StorageLatencyBenchmark, StorageLatencyLiteBenchmark, ThreadWakeBenchmark,
};
use crate::benchmarks::memory::{MemoryBandwidthBenchmark, MemoryLatencyBenchmark};
use crate::benchmarks::Benchmark;
use crate::cloud::CloudClient;
use crate::core::{
    BenchmarkMessage, BenchmarkRunner, RecommendationEngine, RecommendationsReport,
    SystemCheckResult, SystemChecker, SystemInfoCollector,
};
use crate::models::{BenchmarkRun, SystemInfo};
use crate::storage::HistoryStorage;
use crate::ui::views::{
    HistoryAction, HistoryView, HomeAction, HomeView, PreCheckAction, PreCheckView,
    ResultsAction, ResultsView, RunningView,
};
use crate::ui::Theme;

// SHA-256 hash of the admin password
const ADMIN_PASSWORD_HASH: &str = "92f71e72f53a12f3851825f1caf01587679bc8333ecf07c9df745b0c4386eec0";

fn verify_admin_password(password: &str) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    let hash_hex = format!("{:x}", result);
    hash_hex == ADMIN_PASSWORD_HASH
}

/// Application state
#[derive(Debug, Clone, PartialEq, Eq)]
enum AppState {
    Home,
    PreCheck,
    Running,
    Results,
    History,
    ViewingHistoricRun(usize),       // Index of run to view
}

/// Main application
pub struct WorkBenchProApp {
    state: AppState,
    system_info: SystemInfo,
    runner: BenchmarkRunner,
    receiver: Option<Receiver<BenchmarkMessage>>,

    // Running state
    overall_progress: f32,
    current_test_progress: f32,
    current_test: String,
    current_message: String,
    completed_tests: Vec<String>,

    // Pre-check
    system_check: Option<SystemCheckResult>,
    system_check_receiver: Option<Receiver<SystemCheckResult>>,

    // Results
    last_run: Option<BenchmarkRun>,
    recommendations: Option<RecommendationsReport>,

    // History
    history_storage: HistoryStorage,
    history_runs: Vec<BenchmarkRun>,

    // Cloud/Community
    cloud_client: CloudClient,

    // Upload dialog state
    show_upload_dialog: bool,
    upload_display_name: String,
    upload_user_name: String,
    upload_description: String, // Optional description (VDI, Desktop, Laptop, etc.)
    upload_in_progress: bool,
    upload_error: Option<String>,
    upload_success: bool,
    upload_run_index: Option<usize>, // Index of run being uploaded (None = last_run)

    // Window resize flag (for deferred resize after benchmark completes)
    pending_window_resize: Option<egui::Vec2>,

    // Remove upload dialog state
    show_remove_upload_dialog: bool,
    remove_upload_password: String,
    remove_upload_error: Option<String>,
    remove_upload_run_index: Option<usize>,

    // Delete confirmation dialog state
    show_delete_dialog: bool,
    delete_run_index: Option<usize>,
    delete_also_cloud: bool,
    delete_password: String,
    delete_error: Option<String>,
    delete_found_remote_id: Option<String>, // Remote ID found by matching, for runs without local remote_id

    // Save error (for debugging)
    last_save_error: Option<String>,

    // Safe mode - skip AV-sensitive tests
    safe_mode: bool,
}

impl WorkBenchProApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Apply theme
        Theme::apply(&cc.egui_ctx);

        // Collect system info
        let system_info = SystemInfoCollector::collect();
        let machine_name = system_info.hostname.clone();

        // Load history
        let mut history_storage = HistoryStorage::new();
        let history_runs = history_storage.load_all().unwrap_or_default();

        Self {
            state: AppState::Home,
            system_info,
            runner: BenchmarkRunner::new(),
            receiver: None,
            overall_progress: 0.0,
            current_test_progress: 0.0,
            current_test: String::new(),
            current_message: String::new(),
            completed_tests: Vec::new(),
            system_check: None,
            system_check_receiver: None,
            last_run: None,
            recommendations: None,
            history_storage,
            history_runs,
            // Cloud state
            cloud_client: CloudClient::new(),
            // Upload dialog
            show_upload_dialog: false,
            upload_display_name: machine_name,
            upload_user_name: String::new(),
            upload_description: String::new(),
            upload_in_progress: false,
            upload_error: None,
            upload_success: false,
            upload_run_index: None,

            // Window resize
            pending_window_resize: None,

            // Remove upload dialog
            show_remove_upload_dialog: false,
            remove_upload_password: String::new(),
            remove_upload_error: None,
            remove_upload_run_index: None,

            // Delete confirmation dialog
            show_delete_dialog: false,
            delete_run_index: None,
            delete_also_cloud: false,
            delete_password: String::new(),
            delete_error: None,
            delete_found_remote_id: None,

            // Save error
            last_save_error: None,

            // Safe mode disabled by default
            safe_mode: false,
        }
    }

    fn run_system_check(&mut self) {
        // Clear previous check result
        self.system_check = None;

        // Create channel for receiving check result
        let (tx, rx) = mpsc::channel();
        self.system_check_receiver = Some(rx);

        // Run check in background thread
        std::thread::spawn(move || {
            let result = SystemChecker::check();
            let _ = tx.send(result);
        });

        self.state = AppState::PreCheck;
    }

    fn process_system_check(&mut self) {
        if let Some(ref rx) = self.system_check_receiver {
            if let Ok(result) = rx.try_recv() {
                self.system_check = Some(result);
                self.system_check_receiver = None;
            }
        }
    }

    fn start_benchmark(&mut self) {
        // Create benchmark instances
        // In safe mode, skip tests that may trigger antivirus behavioral detection:
        // - FileEnumeration (creates 30,000 files)
        // - ArchiveOps (creates files + tar compression)
        // - PowerShell (executes scripts)
        let mut benchmarks: Vec<Box<dyn Benchmark>> = vec![];

        // Project Operations (disk + file operations)
        if !self.safe_mode {
            benchmarks.push(Box::new(FileEnumerationBenchmark::new()));
        }
        // Use Lite variants in safe mode (smaller files, different test IDs for DB tracking)
        if self.safe_mode {
            benchmarks.push(Box::new(RandomReadLiteBenchmark::new()));
        } else {
            benchmarks.push(Box::new(RandomReadBenchmark::new()));
        }
        benchmarks.push(Box::new(MetadataOpsBenchmark::new()));
        benchmarks.push(Box::new(TraversalBenchmark::new()));
        if self.safe_mode {
            benchmarks.push(Box::new(LargeFileReadLiteBenchmark::new()));
        } else {
            benchmarks.push(Box::new(LargeFileReadBenchmark::new()));
        }
        benchmarks.push(Box::new(RobocopyBenchmark::new()));
        benchmarks.push(Box::new(WindowsSearchBenchmark::new()));
        benchmarks.push(Box::new(DefenderImpactBenchmark::new()));

        // Build Performance (CPU + real app benchmarks)
        benchmarks.push(Box::new(SingleThreadBenchmark::new()));
        benchmarks.push(Box::new(MultiThreadBenchmark::new()));
        benchmarks.push(Box::new(MixedWorkloadBenchmark::new()));
        benchmarks.push(Box::new(SustainedWriteBenchmark::new()));
        if !self.safe_mode {
            benchmarks.push(Box::new(ArchiveOpsBenchmark::new()));
        }
        benchmarks.push(Box::new(WindowsCompressionBenchmark::new()));
        if !self.safe_mode {
            benchmarks.push(Box::new(PowerShellBenchmark::new()));
        }

        // Responsiveness (latency + memory benchmarks)
        if self.safe_mode {
            benchmarks.push(Box::new(StorageLatencyLiteBenchmark::new()));
        } else {
            benchmarks.push(Box::new(StorageLatencyBenchmark::new()));
        }
        benchmarks.push(Box::new(ProcessSpawnBenchmark::new()));
        benchmarks.push(Box::new(ThreadWakeBenchmark::new()));
        benchmarks.push(Box::new(MemoryLatencyBenchmark::new()));
        benchmarks.push(Box::new(MemoryBandwidthBenchmark::new()));

        // Windows System Tools
        benchmarks.push(Box::new(RegistryBenchmark::new()));
        benchmarks.push(Box::new(EventLogBenchmark::new()));
        benchmarks.push(Box::new(TaskSchedulerBenchmark::new()));
        benchmarks.push(Box::new(AppLaunchBenchmark::new()));
        benchmarks.push(Box::new(ServicesBenchmark::new()));
        benchmarks.push(Box::new(WmicBenchmark::new()));
        benchmarks.push(Box::new(ProcessesBenchmark::new()));
        benchmarks.push(Box::new(SymlinkBenchmark::new()));
        benchmarks.push(Box::new(EnvironmentBenchmark::new()));

        // Reset running state
        self.overall_progress = 0.0;
        self.current_test_progress = 0.0;
        self.current_test = String::new();
        self.current_message = "Starting...".to_string();
        self.completed_tests.clear();

        // Start runner
        let receiver = self.runner.start(benchmarks);
        self.receiver = Some(receiver);
        self.state = AppState::Running;
    }

    fn cancel_benchmark(&mut self, ctx: &egui::Context) {
        self.runner.cancel();
        self.state = AppState::Home;
        self.receiver = None;
        // Reset window size
        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(755.0, 400.0)));
    }

    /// Start a single benchmark by index (debug feature)
    #[cfg(feature = "debug-logging")]
    fn start_single_benchmark(&mut self, index: usize) {
        // Create all benchmarks in order (must match the order in home.rs TestSpecs)
        let all_benchmarks: Vec<Box<dyn Benchmark>> = vec![
            // Project Operations (0-7)
            Box::new(FileEnumerationBenchmark::new()),   // 0
            Box::new(RandomReadBenchmark::new()),        // 1
            Box::new(MetadataOpsBenchmark::new()),       // 2
            Box::new(TraversalBenchmark::new()),         // 3
            Box::new(LargeFileReadBenchmark::new()),     // 4
            Box::new(RobocopyBenchmark::new()),          // 5
            Box::new(WindowsSearchBenchmark::new()),     // 6
            Box::new(DefenderImpactBenchmark::new()),    // 7
            // Build Performance (8-14)
            Box::new(SingleThreadBenchmark::new()),      // 8
            Box::new(MultiThreadBenchmark::new()),       // 9
            Box::new(MixedWorkloadBenchmark::new()),     // 10
            Box::new(SustainedWriteBenchmark::new()),    // 11
            Box::new(ArchiveOpsBenchmark::new()),        // 12
            Box::new(WindowsCompressionBenchmark::new()),// 13
            Box::new(PowerShellBenchmark::new()),        // 14
            // Responsiveness (15-19)
            Box::new(StorageLatencyBenchmark::new()),    // 15
            Box::new(ProcessSpawnBenchmark::new()),      // 16
            Box::new(ThreadWakeBenchmark::new()),        // 17
            Box::new(MemoryLatencyBenchmark::new()),     // 18
            Box::new(MemoryBandwidthBenchmark::new()),   // 19
            // Windows System Tools (20-28)
            Box::new(RegistryBenchmark::new()),          // 20
            Box::new(EventLogBenchmark::new()),          // 21
            Box::new(TaskSchedulerBenchmark::new()),     // 22
            Box::new(AppLaunchBenchmark::new()),         // 23
            Box::new(ServicesBenchmark::new()),          // 24
            Box::new(WmicBenchmark::new()),              // 25
            Box::new(ProcessesBenchmark::new()),         // 26
            Box::new(SymlinkBenchmark::new()),           // 27
            Box::new(EnvironmentBenchmark::new()),       // 28
        ];

        // Get the single benchmark at the requested index
        if index >= all_benchmarks.len() {
            tracing::error!("Invalid benchmark index: {}", index);
            return;
        }

        // Take just the one benchmark we want
        let benchmark = all_benchmarks.into_iter().nth(index).unwrap();
        let benchmark_name = benchmark.name().to_string();

        tracing::info!("Starting single benchmark test: [{}] {}", index, benchmark_name);

        // Reset running state
        self.overall_progress = 0.0;
        self.current_test_progress = 0.0;
        self.current_test = String::new();
        self.current_message = format!("Starting single test: {}...", benchmark_name);
        self.completed_tests.clear();

        // Start runner with just this one benchmark
        let receiver = self.runner.start(vec![benchmark]);
        self.receiver = Some(receiver);
        self.state = AppState::Running;
    }

    fn process_messages(&mut self) {
        let receiver = self.receiver.take();

        if let Some(rx) = receiver {
            let mut should_keep_receiver = true;

            while let Ok(msg) = rx.try_recv() {
                match msg {
                    BenchmarkMessage::Progress {
                        benchmark_id: _,
                        overall_progress,
                        test_progress,
                        message,
                    } => {
                        self.overall_progress = overall_progress;
                        self.current_test_progress = test_progress;
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
                        match self.history_storage.save(&run) {
                            Ok(path) => {
                                tracing::info!("Saved to: {}", path.display());
                                self.last_save_error = None;
                            }
                            Err(e) => {
                                let err_msg = format!("Failed to save: {}", e);
                                tracing::error!("{}", err_msg);
                                self.last_save_error = Some(err_msg);
                            }
                        }

                        // Generate recommendations
                        // TODO: In future, fetch percentile ranks from cloud for comparison
                        self.recommendations = Some(RecommendationEngine::analyze(&run, None));

                        self.last_run = Some(*run);
                        self.state = AppState::Results;
                        should_keep_receiver = false;

                        // Results view will resize itself based on content

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
    }

    fn open_delete_dialog(&mut self, idx: usize) {
        self.delete_run_index = Some(idx);
        self.delete_also_cloud = false;
        self.delete_password = String::new();
        self.delete_error = None;
        self.delete_found_remote_id = None;

        // If the run doesn't have a remote_id locally, check if it exists in the cloud
        if let Some(run) = self.history_runs.get(idx) {
            if run.remote_id.is_none() {
                // Try to find a matching record in the cloud
                if let Ok(Some(remote_id)) = self.cloud_client.find_matching(&run.machine_name, &run.timestamp) {
                    self.delete_found_remote_id = Some(remote_id);
                }
            }
        }

        self.show_delete_dialog = true;
    }

    fn reset_delete_dialog(&mut self) {
        self.show_delete_dialog = false;
        self.delete_run_index = None;
        self.delete_also_cloud = false;
        self.delete_password = String::new();
        self.delete_error = None;
        self.delete_found_remote_id = None;
    }

    fn execute_delete(&mut self) {
        let Some(idx) = self.delete_run_index else {
            return;
        };

        if idx >= self.history_runs.len() {
            self.reset_delete_dialog();
            return;
        }

        let run = &self.history_runs[idx];

        // Determine if there's a cloud record (either from local remote_id or found by matching)
        let cloud_remote_id = run.remote_id.clone().or_else(|| self.delete_found_remote_id.clone());
        let has_cloud_record = cloud_remote_id.is_some();

        // If user wants to delete from cloud too - no password needed since having the local file proves ownership
        if self.delete_also_cloud && has_cloud_record {
            if let Some(ref remote_id) = cloud_remote_id {
                match CloudClient::new().delete(remote_id) {
                    Ok(_) => {}
                    Err(e) => {
                        self.delete_error = Some(format!("Failed to delete from cloud: {}", e));
                        return;
                    }
                }
            }
        }

        // Delete local file
        if let Err(e) = self.history_storage.delete(run) {
            self.delete_error = Some(format!("Failed to delete local file: {}", e));
            return;
        }

        self.reset_delete_dialog();
        self.reload_history();
    }

    fn open_remove_upload_dialog(&mut self, idx: usize) {
        self.remove_upload_run_index = Some(idx);
        self.remove_upload_password = String::new();
        self.remove_upload_error = None;
        self.show_remove_upload_dialog = true;
    }

    fn reset_remove_upload_dialog(&mut self) {
        self.show_remove_upload_dialog = false;
        self.remove_upload_password = String::new();
        self.remove_upload_error = None;
        self.remove_upload_run_index = None;
    }

    fn remove_upload(&mut self) {
        // Verify password first
        if !verify_admin_password(&self.remove_upload_password) {
            self.remove_upload_error = Some("Invalid admin password".to_string());
            return;
        }

        if let Some(idx) = self.remove_upload_run_index {
            if idx < self.history_runs.len() {
                let run = &self.history_runs[idx];

                // Delete from cloud if remote_id exists
                if let Some(ref remote_id) = run.remote_id {
                    if let Err(e) = self.cloud_client.delete(remote_id) {
                        self.remove_upload_error = Some(format!("Failed to remove: {}", e));
                        return;
                    }
                }

                // Clear upload status locally
                if let Some(history_run) = self.history_runs.get_mut(idx) {
                    history_run.remote_id = None;
                    history_run.uploaded_at = None;

                    // Re-save to persist the change
                    if let Err(e) = self.history_storage.save(history_run) {
                        tracing::error!("Failed to update history after removing upload: {}", e);
                    }
                }

                tracing::info!("Upload removed successfully");
                self.reset_remove_upload_dialog();
            }
        }
    }

    fn upload_run(&mut self, run: &BenchmarkRun) {
        self.upload_in_progress = true;
        self.upload_error = None;

        let user_name = if self.upload_user_name.trim().is_empty() {
            None
        } else {
            Some(self.upload_user_name.trim().to_string())
        };
        let description = if self.upload_description.trim().is_empty() {
            None
        } else {
            Some(self.upload_description.trim().to_string())
        };
        match self.cloud_client.upload(run, &self.upload_display_name, user_name, description) {
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
        self.upload_display_name = self.system_info.hostname.clone();
        self.upload_user_name = String::new();
        self.upload_description = String::new();
    }
}

impl eframe::App for WorkBenchProApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Process any pending messages from the benchmark runner
        self.process_messages();

        // Process system check if running
        self.process_system_check();

        // Process any pending window resize
        if let Some(size) = self.pending_window_resize.take() {
            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(size));
        }

        // Determine actions needed
        let mut home_action = HomeAction::None;
        let mut precheck_action = PreCheckAction::None;
        let mut action_cancel = false;
        let mut results_action = ResultsAction::None;
        let mut history_action = HistoryAction::None;
        let mut historic_view_back = false;

        egui::CentralPanel::default().show(ctx, |ui| {
            match &self.state {
                AppState::Home => {
                    home_action = HomeView::show(ui, &self.system_info, &mut self.safe_mode);
                }
                AppState::PreCheck => {
                    precheck_action = PreCheckView::show(ui, self.system_check.as_ref());
                }
                AppState::Running => {
                    action_cancel = RunningView::show(
                        ui,
                        self.overall_progress,
                        self.current_test_progress,
                        &self.current_test,
                        &self.current_message,
                        &self.completed_tests,
                    );
                }
                AppState::Results => {
                    if let Some(run) = &self.last_run {
                        results_action = ResultsView::show_with_save(ui, run, self.recommendations.as_ref());
                    }
                }
                AppState::History => {
                    history_action = HistoryView::show(
                        ui,
                        &self.history_runs,
                        Some(self.history_storage.storage_path()),
                        self.last_save_error.as_deref(),
                        self.history_storage.last_load_stats.as_ref(),
                    );
                }
                AppState::ViewingHistoricRun(idx) => {
                    if let Some(run) = self.history_runs.get(*idx) {
                        historic_view_back = ResultsView::show(ui, run);
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
                                    .hint_text("Name for your submission (or leave empty)"),
                            );

                            ui.add_space(8.0);
                            ui.label(
                                egui::RichText::new("User name (optional):")
                                    .size(Theme::SIZE_CAPTION)
                                    .color(Theme::TEXT_SECONDARY),
                            );
                            ui.add(
                                egui::TextEdit::singleline(&mut self.upload_user_name)
                                    .desired_width(280.0)
                                    .hint_text("Your name or alias"),
                            );

                            ui.add_space(8.0);
                            ui.label(
                                egui::RichText::new("Description (optional):")
                                    .size(Theme::SIZE_CAPTION)
                                    .color(Theme::TEXT_SECONDARY),
                            );
                            ui.add(
                                egui::TextEdit::singleline(&mut self.upload_description)
                                    .desired_width(280.0)
                                    .hint_text("e.g. VDI, Desktop, Laptop, Gaming PC"),
                            );

                            ui.add_space(12.0);
                            ui.separator();
                            ui.add_space(8.0);

                            // Privacy notice
                            ui.label(
                                egui::RichText::new("Privacy notice")
                                    .size(Theme::SIZE_CAPTION)
                                    .color(Theme::TEXT_PRIMARY)
                                    .strong(),
                            );
                            ui.add_space(4.0);
                            ui.label(
                                egui::RichText::new("Only benchmark measurements and basic hardware info \n(CPU, RAM, storage type) are uploaded. No files, personal \ndata, or identifying information is collected. All fields \nabove are optional — your upload can be fully anonymous.")
                                    .size(Theme::SIZE_CAPTION)
                                    .color(Theme::TEXT_SECONDARY),
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

                                if ui.add(upload_btn).clicked() {
                                    // Use "Anonymous" if no display name provided
                                    if self.upload_display_name.trim().is_empty() {
                                        self.upload_display_name = "Anonymous".to_string();
                                    }
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

        // Show remove upload dialog if active
        let mut remove_should_close = false;
        let mut remove_should_confirm = false;

        if self.show_remove_upload_dialog {
            egui::Window::new("Remove Upload")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.set_min_width(300.0);

                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new("Admin authentication required")
                                .size(Theme::SIZE_BODY)
                                .color(Theme::WARNING),
                        );
                        ui.add_space(8.0);

                        ui.label(
                            egui::RichText::new("Enter admin password:")
                                .size(Theme::SIZE_CAPTION)
                                .color(Theme::TEXT_SECONDARY),
                        );
                        let response = ui.add(
                            egui::TextEdit::singleline(&mut self.remove_upload_password)
                                .password(true)
                                .desired_width(280.0)
                                .hint_text("Admin password"),
                        );

                        // Submit on Enter key
                        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            remove_should_confirm = true;
                        }

                        if let Some(ref err) = self.remove_upload_error {
                            ui.add_space(8.0);
                            ui.label(
                                egui::RichText::new(err)
                                    .size(Theme::SIZE_CAPTION)
                                    .color(Theme::ERROR),
                            );
                        }

                        ui.add_space(12.0);

                        ui.horizontal(|ui| {
                            if ui.button("Cancel").clicked() {
                                remove_should_close = true;
                            }

                            ui.add_space(8.0);

                            let remove_btn = egui::Button::new(
                                egui::RichText::new("Remove")
                                    .color(egui::Color32::WHITE),
                            )
                            .fill(Theme::ERROR);

                            if ui.add(remove_btn).clicked() {
                                remove_should_confirm = true;
                            }
                        });
                    });
                });
        }

        if remove_should_close {
            self.reset_remove_upload_dialog();
        }

        if remove_should_confirm {
            self.remove_upload();
        }

        // Show delete confirmation dialog
        let mut delete_should_close = false;
        let mut delete_should_confirm = false;

        if self.show_delete_dialog {
            // Check if there's a cloud record (either local remote_id or found by matching)
            let has_cloud_record = self.delete_run_index
                .and_then(|idx| self.history_runs.get(idx))
                .map(|r| r.remote_id.is_some())
                .unwrap_or(false)
                || self.delete_found_remote_id.is_some();

            egui::Window::new("Delete Benchmark")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.set_min_width(320.0);

                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new("Are you sure you want to delete this benchmark?")
                                .size(Theme::SIZE_BODY),
                        );
                        ui.add_space(8.0);

                        if has_cloud_record {
                            let label = if self.delete_found_remote_id.is_some() {
                                "Also remove from community results (found matching upload)"
                            } else {
                                "Also remove from community results"
                            };
                            ui.checkbox(&mut self.delete_also_cloud, label);
                        }

                        if let Some(ref err) = self.delete_error {
                            ui.add_space(8.0);
                            ui.label(
                                egui::RichText::new(err)
                                    .size(Theme::SIZE_CAPTION)
                                    .color(Theme::ERROR),
                            );
                        }

                        ui.add_space(12.0);

                        ui.horizontal(|ui| {
                            if ui.button("Cancel").clicked() {
                                delete_should_close = true;
                            }

                            ui.add_space(8.0);

                            let delete_btn = egui::Button::new(
                                egui::RichText::new("Delete")
                                    .color(egui::Color32::WHITE),
                            )
                            .fill(Theme::ERROR);

                            if ui.add(delete_btn).clicked() {
                                delete_should_confirm = true;
                            }
                        });
                    });
                });
        }

        if delete_should_close {
            self.reset_delete_dialog();
        }

        if delete_should_confirm {
            self.execute_delete();
        }

        // Handle home actions
        match home_action {
            HomeAction::None => {}
            HomeAction::Run => {
                // Go to pre-check first, not directly to running
                self.run_system_check();
            }
            HomeAction::History => {
                self.reload_history();
                self.state = AppState::History;
            }
            #[cfg(feature = "debug-logging")]
            HomeAction::RunSingleTest(idx) => {
                // Debug feature: run a single test directly (skip pre-check)
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(755.0, 750.0)));
                self.start_single_benchmark(idx);
            }
        }

        // Handle pre-check actions
        match precheck_action {
            PreCheckAction::None => {}
            PreCheckAction::Cancel => {
                self.system_check = None;
                self.system_check_receiver = None;
                self.state = AppState::Home;
                // Reset window size to default
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(755.0, 400.0)));
            }
            PreCheckAction::Recheck => {
                self.run_system_check();
            }
            PreCheckAction::Proceed => {
                // Expand window for benchmark running view
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(755.0, 750.0)));
                self.start_benchmark();
            }
        }

        if action_cancel {
            self.cancel_benchmark(ctx);
        }

        // Handle results actions
        match results_action {
            ResultsAction::None => {}
            ResultsAction::Back => {
                self.state = AppState::Home;
                // Reset window size to default
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(755.0, 400.0)));
            }
            ResultsAction::History => {
                self.reload_history();
                self.state = AppState::History;
            }
            ResultsAction::Upload => {
                self.upload_run_index = None; // Upload last_run
                self.upload_display_name = self.system_info.hostname.clone();
                self.show_upload_dialog = true;
            }
        }
        if historic_view_back {
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
            HistoryAction::Upload(idx) => {
                self.upload_run_index = Some(idx);
                self.upload_display_name = self.system_info.hostname.clone();
                self.show_upload_dialog = true;
            }
            HistoryAction::RemoveUpload(idx) => {
                self.open_remove_upload_dialog(idx);
            }
            HistoryAction::DeleteRun(idx) => {
                self.open_delete_dialog(idx);
            }
        }
    }
}
