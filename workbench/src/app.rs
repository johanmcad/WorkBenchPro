use std::sync::mpsc::Receiver;

use eframe::egui;
use sha2::{Sha256, Digest};

use crate::benchmarks::apps::{
    AppLaunchBenchmark, ArchiveOpsBenchmark, CSharpCompileBenchmark, DefenderImpactBenchmark,
    EnvironmentBenchmark, EventLogBenchmark, PowerShellBenchmark,
    ProcessesBenchmark, RegistryBenchmark, RobocopyBenchmark, ServicesBenchmark, SymlinkBenchmark,
    TaskSchedulerBenchmark, WindowsCompressionBenchmark, WindowsSearchBenchmark, WmicBenchmark,
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
use crate::cloud::CloudClient;
use crate::core::{BenchmarkMessage, BenchmarkRunner, SystemInfoCollector};
use crate::models::{BenchmarkRun, SystemInfo};
use crate::storage::HistoryStorage;
use crate::ui::views::{
    HistoryAction, HistoryView, HomeAction, HomeView,
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

    // Results
    last_run: Option<BenchmarkRun>,

    // History
    history_storage: HistoryStorage,
    history_runs: Vec<BenchmarkRun>,

    // Cloud/Community
    cloud_client: CloudClient,

    // Upload dialog state
    show_upload_dialog: bool,
    upload_display_name: String,
    upload_in_progress: bool,
    upload_error: Option<String>,
    upload_success: bool,
    upload_run_index: Option<usize>, // Index of run being uploaded (None = last_run)

    // Remove upload dialog state
    show_remove_upload_dialog: bool,
    remove_upload_password: String,
    remove_upload_error: Option<String>,
    remove_upload_run_index: Option<usize>,
}

impl WorkBenchProApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Apply theme
        Theme::apply(&cc.egui_ctx);

        // Collect system info
        let system_info = SystemInfoCollector::collect();
        let machine_name = system_info.hostname.clone();

        // Load history
        let history_storage = HistoryStorage::new();
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
            last_run: None,
            history_storage,
            history_runs,
            // Cloud state
            cloud_client: CloudClient::new(),
            // Upload dialog
            show_upload_dialog: false,
            upload_display_name: machine_name,
            upload_in_progress: false,
            upload_error: None,
            upload_success: false,
            upload_run_index: None,

            // Remove upload dialog
            show_remove_upload_dialog: false,
            remove_upload_password: String::new(),
            remove_upload_error: None,
            remove_upload_run_index: None,
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
            Box::new(WindowsCompressionBenchmark::new()),
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
            Box::new(WmicBenchmark::new()),
            Box::new(ProcessesBenchmark::new()),
            Box::new(SymlinkBenchmark::new()),
            Box::new(EnvironmentBenchmark::new()),
        ];

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
        self.upload_display_name = self.system_info.hostname.clone();
    }
}

impl eframe::App for WorkBenchProApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Process any pending messages from the benchmark runner
        self.process_messages();

        // Determine actions needed
        let mut home_action = HomeAction::None;
        let mut action_cancel = false;
        let mut results_action = ResultsAction::None;
        let mut history_action = HistoryAction::None;
        let mut historic_view_back = false;

        egui::CentralPanel::default().show(ctx, |ui| {
            match &self.state {
                AppState::Home => {
                    home_action = HomeView::show(ui, &self.system_info);
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
                        results_action = ResultsView::show_with_save(ui, run);
                    }
                }
                AppState::History => {
                    history_action = HistoryView::show(ui, &self.history_runs);
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

        // Handle home actions
        match home_action {
            HomeAction::None => {}
            HomeAction::Run => {
                self.start_benchmark();
            }
            HomeAction::History => {
                self.reload_history();
                self.state = AppState::History;
            }
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
                self.delete_history_run(idx);
            }
        }
    }
}
