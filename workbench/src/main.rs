#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod benchmarks;
mod core;
mod export;
mod models;
mod scoring;
mod ui;

use anyhow::Result;
use eframe::egui;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    tracing::info!("Starting WorkBench");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("WorkBench - Developer Workstation Benchmark"),
        ..Default::default()
    };

    eframe::run_native(
        "WorkBench",
        options,
        Box::new(|cc| Ok(Box::new(app::WorkBenchApp::new(cc)))),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run application: {}", e))
}
