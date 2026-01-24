use egui::{Align, Layout, RichText, Ui};

use crate::core::RunConfig;
use crate::models::SystemInfo;
use crate::ui::Theme;

pub struct HomeView;

impl HomeView {
    /// Returns true if the Run button was clicked
    pub fn show(ui: &mut Ui, system_info: &SystemInfo, config: &mut RunConfig) -> bool {
        let mut run_clicked = false;

        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            ui.add_space(20.0);

            // Title
            ui.label(RichText::new("WorkBench").size(32.0).strong().color(Theme::ACCENT));
            ui.label(
                RichText::new("Developer Workstation Benchmark")
                    .size(16.0)
                    .color(Theme::TEXT_SECONDARY),
            );

            ui.add_space(30.0);

            // System Info Card
            egui::Frame::none()
                .fill(Theme::BG_CARD)
                .stroke(egui::Stroke::new(1.0, Theme::BORDER))
                .rounding(8.0)
                .inner_margin(20.0)
                .show(ui, |ui| {
                    ui.set_min_width(500.0);

                    ui.label(RichText::new("System Information").size(18.0).strong());
                    ui.add_space(12.0);

                    egui::Grid::new("system_info_grid")
                        .num_columns(2)
                        .spacing([40.0, 8.0])
                        .show(ui, |ui| {
                            ui.label(RichText::new("Hostname:").color(Theme::TEXT_SECONDARY));
                            ui.label(&system_info.hostname);
                            ui.end_row();

                            ui.label(RichText::new("CPU:").color(Theme::TEXT_SECONDARY));
                            ui.label(&system_info.cpu.name);
                            ui.end_row();

                            ui.label(RichText::new("Cores / Threads:").color(Theme::TEXT_SECONDARY));
                            ui.label(format!(
                                "{} / {}",
                                system_info.cpu.cores, system_info.cpu.threads
                            ));
                            ui.end_row();

                            ui.label(RichText::new("Memory:").color(Theme::TEXT_SECONDARY));
                            ui.label(format!("{:.1} GB", system_info.memory.total_gb()));
                            ui.end_row();

                            ui.label(RichText::new("OS:").color(Theme::TEXT_SECONDARY));
                            ui.label(format!(
                                "{} {}",
                                system_info.os.name, system_info.os.version
                            ));
                            ui.end_row();

                            if !system_info.storage.is_empty() {
                                ui.label(RichText::new("Storage:").color(Theme::TEXT_SECONDARY));
                                let storage = &system_info.storage[0];
                                ui.label(format!(
                                    "{} ({}) - {:.0} GB",
                                    storage.name,
                                    storage.device_type.label(),
                                    storage.capacity_gb()
                                ));
                                ui.end_row();
                            }
                        });
                });

            ui.add_space(30.0);

            // Test Configuration
            egui::Frame::none()
                .fill(Theme::BG_CARD)
                .stroke(egui::Stroke::new(1.0, Theme::BORDER))
                .rounding(8.0)
                .inner_margin(20.0)
                .show(ui, |ui| {
                    ui.set_min_width(500.0);

                    ui.label(RichText::new("Test Configuration").size(18.0).strong());
                    ui.add_space(12.0);

                    ui.checkbox(
                        &mut config.run_project_operations,
                        "Project Operations (file enumeration, random reads)",
                    );
                    ui.checkbox(
                        &mut config.run_build_performance,
                        "Build Performance (compute, sustained write)",
                    );
                    ui.checkbox(
                        &mut config.run_responsiveness,
                        "Responsiveness (latency, memory bandwidth)",
                    );
                    ui.checkbox(&mut config.run_graphics, "Graphics (optional)");
                });

            ui.add_space(30.0);

            // Run Button
            let button = egui::Button::new(RichText::new("Run Benchmark").size(18.0).strong())
                .min_size(egui::vec2(200.0, 50.0))
                .fill(Theme::ACCENT);

            if ui.add(button).clicked() {
                run_clicked = true;
            }

            ui.add_space(20.0);
        });

        run_clicked
    }

    /// Returns (run_clicked, history_clicked)
    pub fn show_with_history(
        ui: &mut Ui,
        system_info: &SystemInfo,
        config: &mut RunConfig,
    ) -> (bool, bool) {
        let mut run_clicked = false;
        let mut history_clicked = false;

        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            ui.add_space(20.0);

            // Title
            ui.label(RichText::new("WorkBench").size(32.0).strong().color(Theme::ACCENT));
            ui.label(
                RichText::new("Developer Workstation Benchmark")
                    .size(16.0)
                    .color(Theme::TEXT_SECONDARY),
            );

            ui.add_space(30.0);

            // System Info Card
            egui::Frame::none()
                .fill(Theme::BG_CARD)
                .stroke(egui::Stroke::new(1.0, Theme::BORDER))
                .rounding(8.0)
                .inner_margin(20.0)
                .show(ui, |ui| {
                    ui.set_min_width(500.0);

                    ui.label(RichText::new("System Information").size(18.0).strong());
                    ui.add_space(12.0);

                    egui::Grid::new("system_info_grid_hist")
                        .num_columns(2)
                        .spacing([40.0, 8.0])
                        .show(ui, |ui| {
                            ui.label(RichText::new("Hostname:").color(Theme::TEXT_SECONDARY));
                            ui.label(&system_info.hostname);
                            ui.end_row();

                            ui.label(RichText::new("CPU:").color(Theme::TEXT_SECONDARY));
                            ui.label(&system_info.cpu.name);
                            ui.end_row();

                            ui.label(RichText::new("Cores / Threads:").color(Theme::TEXT_SECONDARY));
                            ui.label(format!(
                                "{} / {}",
                                system_info.cpu.cores, system_info.cpu.threads
                            ));
                            ui.end_row();

                            ui.label(RichText::new("Memory:").color(Theme::TEXT_SECONDARY));
                            ui.label(format!("{:.1} GB", system_info.memory.total_gb()));
                            ui.end_row();

                            ui.label(RichText::new("OS:").color(Theme::TEXT_SECONDARY));
                            ui.label(format!(
                                "{} {}",
                                system_info.os.name, system_info.os.version
                            ));
                            ui.end_row();

                            if !system_info.storage.is_empty() {
                                ui.label(RichText::new("Storage:").color(Theme::TEXT_SECONDARY));
                                let storage = &system_info.storage[0];
                                ui.label(format!(
                                    "{} ({}) - {:.0} GB",
                                    storage.name,
                                    storage.device_type.label(),
                                    storage.capacity_gb()
                                ));
                                ui.end_row();
                            }
                        });
                });

            ui.add_space(30.0);

            // Test Configuration
            egui::Frame::none()
                .fill(Theme::BG_CARD)
                .stroke(egui::Stroke::new(1.0, Theme::BORDER))
                .rounding(8.0)
                .inner_margin(20.0)
                .show(ui, |ui| {
                    ui.set_min_width(500.0);

                    ui.label(RichText::new("Test Configuration").size(18.0).strong());
                    ui.add_space(12.0);

                    ui.checkbox(
                        &mut config.run_project_operations,
                        "Project Operations (file enumeration, random reads)",
                    );
                    ui.checkbox(
                        &mut config.run_build_performance,
                        "Build Performance (compute, sustained write)",
                    );
                    ui.checkbox(
                        &mut config.run_responsiveness,
                        "Responsiveness (latency, memory bandwidth)",
                    );
                    ui.checkbox(&mut config.run_graphics, "Graphics (optional)");
                });

            ui.add_space(30.0);

            // Buttons
            ui.horizontal(|ui| {
                let run_button =
                    egui::Button::new(RichText::new("Run Benchmark").size(18.0).strong())
                        .min_size(egui::vec2(200.0, 50.0))
                        .fill(Theme::ACCENT);

                if ui.add(run_button).clicked() {
                    run_clicked = true;
                }

                ui.add_space(20.0);

                let history_button =
                    egui::Button::new(RichText::new("View History").size(16.0))
                        .min_size(egui::vec2(150.0, 50.0));

                if ui.add(history_button).clicked() {
                    history_clicked = true;
                }
            });

            ui.add_space(20.0);
        });

        (run_clicked, history_clicked)
    }
}
