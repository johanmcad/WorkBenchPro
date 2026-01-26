#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod benchmarks;
mod cloud;
mod core;
mod export;
mod models;
mod storage;
mod ui;

use anyhow::Result;
use eframe::egui;
use std::path::PathBuf;

// Embed SwiftShader files directly in the executable
#[cfg(windows)]
static SWIFTSHADER_DLL: &[u8] = include_bytes!("../swiftshader/vk_swiftshader.dll");

/// Extract embedded SwiftShader files and configure Vulkan to use them
#[cfg(windows)]
fn setup_swiftshader() {
    let swiftshader_dir = get_swiftshader_dir();

    if std::fs::create_dir_all(&swiftshader_dir).is_err() {
        return;
    }

    let dll_path = swiftshader_dir.join("vk_swiftshader.dll");
    let icd_path = swiftshader_dir.join("vk_swiftshader_icd.json");

    // Extract DLL if not present or different size
    let should_extract_dll = !dll_path.exists()
        || std::fs::metadata(&dll_path)
            .map(|m| m.len() as usize != SWIFTSHADER_DLL.len())
            .unwrap_or(true);

    if should_extract_dll {
        if std::fs::write(&dll_path, SWIFTSHADER_DLL).is_err() {
            return;
        }
    }

    // Create ICD JSON pointing to the DLL
    let icd_content = format!(
        r#"{{"file_format_version": "1.0.0", "ICD": {{"library_path": "{}", "api_version": "1.0.5"}}}}"#,
        dll_path.to_string_lossy().replace('\\', "\\\\")
    );

    if std::fs::write(&icd_path, &icd_content).is_err() {
        return;
    }

    // Configure Vulkan to use SwiftShader
    std::env::set_var("VK_ICD_FILENAMES", &icd_path);
    std::env::set_var("VK_DRIVER_FILES", &icd_path);
}

#[cfg(not(windows))]
fn setup_swiftshader() {
    // SwiftShader embedding only supported on Windows
}

/// Get directory for SwiftShader files (always uses %TEMP%)
fn get_swiftshader_dir() -> PathBuf {
    std::env::temp_dir().join("workbench_pro_swiftshader")
}

fn main() -> Result<()> {
    // Extract and configure SwiftShader for software rendering
    setup_swiftshader();

    // Check environment variable to force a specific renderer
    let use_glow = std::env::var("WORKBENCH_PRO_RENDERER")
        .map(|v| {
            v.to_lowercase() == "glow"
                || v.to_lowercase() == "gl"
                || v.to_lowercase() == "opengl"
        })
        .unwrap_or(false);

    let renderer = if use_glow {
        eframe::Renderer::Glow
    } else {
        eframe::Renderer::Wgpu
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("WorkBench-Pro - Developer Workstation Benchmark"),
        renderer,
        wgpu_options: eframe::egui_wgpu::WgpuConfiguration {
            #[cfg(windows)]
            supported_backends: eframe::wgpu::Backends::DX12
                | eframe::wgpu::Backends::VULKAN
                | eframe::wgpu::Backends::GL,
            #[cfg(not(windows))]
            supported_backends: eframe::wgpu::Backends::all(),
            power_preference: eframe::wgpu::PowerPreference::None,
            device_descriptor: std::sync::Arc::new(|_adapter| eframe::wgpu::DeviceDescriptor {
                label: Some("WorkBench-Pro Device"),
                required_features: eframe::wgpu::Features::empty(),
                required_limits: eframe::wgpu::Limits::downlevel_webgl2_defaults(),
                memory_hints: Default::default(),
            }),
            ..Default::default()
        },
        ..Default::default()
    };

    eframe::run_native(
        "WorkBench-Pro",
        options,
        Box::new(|cc| {
            crate::ui::Theme::apply(&cc.egui_ctx);
            Ok(Box::new(app::WorkBenchProApp::new(cc)))
        }),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run application: {}", e))
}
