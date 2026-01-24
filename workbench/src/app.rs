use std::sync::mpsc::Receiver;

use eframe::egui;

use crate::benchmarks::disk::{
    FileEnumerationBenchmark, LargeFileReadBenchmark, MetadataOpsBenchmark, RandomReadBenchmark,
    TraversalBenchmark,
};
use crate::benchmarks::Benchmark;
use crate::core::{BenchmarkMessage, BenchmarkRunner, RunConfig, SystemInfoCollector};
use crate::export::JsonExporter;
use crate::models::{BenchmarkRun, SystemInfo};
use crate::ui::views::{HomeView, ResultsView, RunningView};
use crate::ui::Theme;

/// Application state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AppState {
    Home,
    Running,
    Results,
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
}

impl WorkBenchApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Apply theme
        Theme::apply(&cc.egui_ctx);

        // Collect system info
        let system_info = SystemInfoCollector::collect();
        let machine_name = system_info.hostname.clone();

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
        }
    }

    fn start_benchmark(&mut self) {
        // Create benchmark instances
        let benchmarks: Vec<Box<dyn Benchmark>> = vec![
            Box::new(FileEnumerationBenchmark::new()),
            Box::new(RandomReadBenchmark::new()),
            Box::new(MetadataOpsBenchmark::new()),
            Box::new(TraversalBenchmark::new()),
            Box::new(LargeFileReadBenchmark::new()),
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
        // Take the receiver temporarily to avoid borrow issues
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
                        self.last_run = Some(*run);
                        self.state = AppState::Results;
                        should_keep_receiver = false;
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

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.state {
                AppState::Home => {
                    action_start = HomeView::show(ui, &self.system_info, &mut self.run_config);
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
                        let (back, export) = ResultsView::show(ui, run);
                        action_back = back;
                        action_export = export;
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
    }
}
