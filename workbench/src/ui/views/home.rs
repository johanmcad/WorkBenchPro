use egui::{Align, Layout, RichText, Ui};

use crate::core::RunConfig;
use crate::models::SystemInfo;
use crate::ui::Theme;

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
        config: &mut RunConfig,
    ) -> (bool, bool) {
        let mut run_clicked = false;
        let mut history_clicked = false;

        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            ui.add_space(32.0);

            // App title + tagline
            ui.label(
                RichText::new("WorkBench")
                    .size(Theme::SIZE_TITLE + 8.0) // Slightly larger for main title
                    .strong()
                    .color(Theme::ACCENT),
            );
            ui.add_space(4.0);
            ui.label(
                RichText::new("Developer Workstation Benchmark")
                    .size(Theme::SIZE_CARD)
                    .color(Theme::TEXT_SECONDARY),
            );

            ui.add_space(32.0);

            // System Info Card
            egui::Frame::none()
                .fill(Theme::BG_CARD)
                .stroke(egui::Stroke::new(1.0, Theme::BORDER))
                .rounding(Theme::CARD_ROUNDING)
                .inner_margin(20.0)
                .show(ui, |ui| {
                    ui.set_min_width(480.0);

                    ui.label(
                        RichText::new("System Information")
                            .size(Theme::SIZE_SECTION)
                            .strong()
                            .color(Theme::TEXT_PRIMARY),
                    );
                    ui.add_space(16.0);

                    egui::Grid::new("system_info_grid")
                        .num_columns(2)
                        .spacing([40.0, 10.0])
                        .show(ui, |ui| {
                            Self::info_row(ui, "Hostname", &system_info.hostname);
                            Self::info_row(ui, "CPU", &system_info.cpu.name);
                            Self::info_row(
                                ui,
                                "Cores / Threads",
                                &format!("{} / {}", system_info.cpu.cores, system_info.cpu.threads),
                            );
                            Self::info_row(
                                ui,
                                "Memory",
                                &format!("{:.1} GB", system_info.memory.total_gb()),
                            );
                            Self::info_row(
                                ui,
                                "OS",
                                &format!("{} {}", system_info.os.name, system_info.os.version),
                            );

                            if !system_info.storage.is_empty() {
                                let storage = &system_info.storage[0];
                                Self::info_row(
                                    ui,
                                    "Storage",
                                    &format!(
                                        "{} ({}) - {:.0} GB",
                                        storage.name,
                                        storage.device_type.label(),
                                        storage.capacity_gb()
                                    ),
                                );
                            }
                        });
                });

            ui.add_space(24.0);

            // Test Configuration Card
            egui::Frame::none()
                .fill(Theme::BG_CARD)
                .stroke(egui::Stroke::new(1.0, Theme::BORDER))
                .rounding(Theme::CARD_ROUNDING)
                .inner_margin(20.0)
                .show(ui, |ui| {
                    ui.set_min_width(480.0);

                    ui.label(
                        RichText::new("Test Configuration")
                            .size(Theme::SIZE_SECTION)
                            .strong()
                            .color(Theme::TEXT_PRIMARY),
                    );
                    ui.add_space(16.0);

                    ui.checkbox(
                        &mut config.run_project_operations,
                        RichText::new("Project Operations").size(Theme::SIZE_BODY),
                    );
                    ui.label(
                        RichText::new("    File enumeration, random reads, metadata ops")
                            .size(Theme::SIZE_CAPTION)
                            .color(Theme::TEXT_SECONDARY),
                    );

                    ui.add_space(8.0);

                    ui.checkbox(
                        &mut config.run_build_performance,
                        RichText::new("Build Performance").size(Theme::SIZE_BODY),
                    );
                    ui.label(
                        RichText::new("    CPU compute, sustained write, compilation")
                            .size(Theme::SIZE_CAPTION)
                            .color(Theme::TEXT_SECONDARY),
                    );

                    ui.add_space(8.0);

                    ui.checkbox(
                        &mut config.run_responsiveness,
                        RichText::new("Responsiveness").size(Theme::SIZE_BODY),
                    );
                    ui.label(
                        RichText::new("    Latency, memory bandwidth, process spawn")
                            .size(Theme::SIZE_CAPTION)
                            .color(Theme::TEXT_SECONDARY),
                    );

                    ui.add_space(8.0);

                    ui.checkbox(
                        &mut config.run_graphics,
                        RichText::new("Graphics (optional)").size(Theme::SIZE_BODY),
                    );
                    ui.label(
                        RichText::new("    GPU performance tests")
                            .size(Theme::SIZE_CAPTION)
                            .color(Theme::TEXT_SECONDARY),
                    );
                });

            ui.add_space(32.0);

            // Buttons
            ui.horizontal(|ui| {
                // Run Button (large, centered, prominent)
                let run_button = egui::Button::new(
                    RichText::new("Run Benchmark")
                        .size(Theme::SIZE_SECTION)
                        .strong()
                        .color(egui::Color32::WHITE),
                )
                .min_size(egui::vec2(200.0, 50.0))
                .fill(Theme::ACCENT)
                .rounding(Theme::CARD_ROUNDING);

                if ui.add(run_button).clicked() {
                    run_clicked = true;
                }

                ui.add_space(16.0);

                // History Button (secondary style)
                let history_button = egui::Button::new(
                    RichText::new("View History").size(Theme::SIZE_BODY),
                )
                .min_size(egui::vec2(120.0, 50.0))
                .rounding(Theme::CARD_ROUNDING);

                if ui.add(history_button).clicked() {
                    history_clicked = true;
                }
            });

            ui.add_space(32.0);
        });

        (run_clicked, history_clicked)
    }

    fn info_row(ui: &mut Ui, label: &str, value: &str) {
        ui.label(
            RichText::new(format!("{}:", label))
                .color(Theme::TEXT_SECONDARY)
                .size(Theme::SIZE_BODY),
        );
        ui.label(RichText::new(value).size(Theme::SIZE_BODY).color(Theme::TEXT_PRIMARY));
        ui.end_row();
    }
}
