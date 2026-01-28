//! Pre-benchmark system check UI view.

use egui::{Align, Layout, RichText, Ui};

use crate::core::system_check::{SystemCheckResult, WarningSeverity};
use crate::ui::Theme;

/// Actions from the pre-check view
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreCheckAction {
    /// No action
    None,
    /// User cancelled - go back to home
    Cancel,
    /// User wants to recheck system state
    Recheck,
    /// User wants to proceed with benchmark (even with warnings)
    Proceed,
}

/// Pre-check view that shows system readiness status
pub struct PreCheckView;

impl PreCheckView {
    /// Show the pre-check view
    /// Returns the action requested by the user
    /// If check_result is None, shows a loading state
    pub fn show(ui: &mut Ui, check_result: Option<&SystemCheckResult>) -> PreCheckAction {
        let mut action = PreCheckAction::None;

        // Calculate needed height based on content
        let needed_height = if let Some(result) = check_result {
            let base_height = 320.0; // Title + status card + buttons + padding
            let processes_height = if !result.high_cpu_processes.is_empty() {
                100.0 + (result.high_cpu_processes.len() as f32 * 24.0)
            } else {
                0.0
            };
            let warnings_height = if !result.warnings.is_empty() {
                80.0 + (result.warnings.len() as f32 * 120.0)
            } else {
                0.0
            };
            base_height + processes_height + warnings_height
        } else {
            300.0 // Loading state
        };

        // Resize window if needed (minimum 420, maximum 750)
        let target_height = needed_height.clamp(420.0, 750.0);
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(755.0, target_height)));

        // Ensure minimum size for all content to be visible
        ui.set_min_width(550.0);
        ui.set_min_height(target_height - 50.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.add_space(12.0);

                // Title
                ui.label(
                    RichText::new("System Check")
                        .size(Theme::SIZE_SECTION)
                        .strong()
                        .color(Theme::ACCENT),
                );

                ui.add_space(4.0);

                // Show loading state if check is still running
                let Some(check_result) = check_result else {
                    ui.add_space(40.0);
                    ui.spinner();
                    ui.add_space(12.0);
                    ui.label(
                        RichText::new("Checking system status...")
                            .size(Theme::SIZE_BODY)
                            .color(Theme::TEXT_SECONDARY),
                    );
                    ui.add_space(8.0);
                    ui.label(
                        RichText::new("Measuring CPU usage over 3 seconds")
                            .size(Theme::SIZE_CAPTION)
                            .color(Theme::TEXT_SECONDARY),
                    );
                    ui.add_space(40.0);

                    // Only show cancel button while loading
                    let cancel_btn = egui::Button::new(
                        RichText::new("Cancel").size(Theme::SIZE_BODY),
                    )
                    .min_size(egui::vec2(80.0, 32.0))
                    .rounding(Theme::CARD_ROUNDING);

                    if ui.add(cancel_btn).clicked() {
                        action = PreCheckAction::Cancel;
                    }

                    // Request repaint to update spinner
                    ui.ctx().request_repaint();
                    return;
                };

                // Subtitle based on result
                if check_result.ready_to_benchmark {
                    ui.label(
                        RichText::new("System is ready for benchmarking")
                            .size(Theme::SIZE_BODY)
                            .color(Theme::SUCCESS),
                    );
                } else {
                    ui.label(
                        RichText::new("Issues detected that may affect results")
                            .size(Theme::SIZE_BODY)
                            .color(Theme::WARNING),
                    );
                }

                ui.add_space(16.0);

                // System Status Overview Card
                egui::Frame::none()
                    .fill(Theme::BG_CARD)
                    .stroke(egui::Stroke::new(1.0, Theme::BORDER))
                    .rounding(Theme::CARD_ROUNDING)
                    .inner_margin(12.0)
                    .show(ui, |ui| {
                        ui.set_min_width(500.0);

                        ui.label(
                            RichText::new("System Status")
                                .size(Theme::SIZE_BODY)
                                .strong()
                                .color(Theme::TEXT_PRIMARY),
                        );
                        ui.add_space(8.0);

                        egui::Grid::new("system_status_grid")
                            .num_columns(2)
                            .spacing([16.0, 6.0])
                            .show(ui, |ui| {
                                // CPU Usage
                                ui.label(
                                    RichText::new("CPU Usage:")
                                        .size(Theme::SIZE_CAPTION)
                                        .color(Theme::TEXT_SECONDARY),
                                );
                                let cpu_color = if check_result.cpu_usage_percent > 50.0 {
                                    Theme::ERROR
                                } else if check_result.cpu_usage_percent > 20.0 {
                                    Theme::WARNING
                                } else {
                                    Theme::SUCCESS
                                };
                                ui.label(
                                    RichText::new(format!("{:.1}%", check_result.cpu_usage_percent))
                                        .size(Theme::SIZE_CAPTION)
                                        .color(cpu_color),
                                );
                                ui.end_row();

                                // Available Memory
                                ui.label(
                                    RichText::new("Available Memory:")
                                        .size(Theme::SIZE_CAPTION)
                                        .color(Theme::TEXT_SECONDARY),
                                );
                                let mem_color = if check_result.available_memory_gb < 2.0 {
                                    Theme::ERROR
                                } else if check_result.available_memory_gb < 4.0 {
                                    Theme::WARNING
                                } else {
                                    Theme::SUCCESS
                                };
                                ui.label(
                                    RichText::new(format!("{:.1} GB", check_result.available_memory_gb))
                                        .size(Theme::SIZE_CAPTION)
                                        .color(mem_color),
                                );
                                ui.end_row();

                                // Power State
                                ui.label(
                                    RichText::new("Power State:")
                                        .size(Theme::SIZE_CAPTION)
                                        .color(Theme::TEXT_SECONDARY),
                                );
                                let (power_text, power_color) = match &check_result.power_state {
                                    crate::core::system_check::PowerState::PluggedIn => {
                                        ("Plugged In".to_string(), Theme::SUCCESS)
                                    }
                                    crate::core::system_check::PowerState::OnBattery(pct) => {
                                        let color = if *pct < 50 {
                                            Theme::ERROR
                                        } else if *pct < 80 {
                                            Theme::WARNING
                                        } else {
                                            Theme::SUCCESS
                                        };
                                        (format!("Battery ({}%)", pct), color)
                                    }
                                    crate::core::system_check::PowerState::Unknown => {
                                        ("Unknown".to_string(), Theme::TEXT_SECONDARY)
                                    }
                                };
                                ui.label(
                                    RichText::new(power_text)
                                        .size(Theme::SIZE_CAPTION)
                                        .color(power_color),
                                );
                                ui.end_row();

                                // Power Plan
                                ui.label(
                                    RichText::new("Power Plan:")
                                        .size(Theme::SIZE_CAPTION)
                                        .color(Theme::TEXT_SECONDARY),
                                );
                                let plan_color = if check_result.power_plan.is_suboptimal() {
                                    if matches!(check_result.power_plan, crate::core::system_check::PowerPlan::PowerSaver) {
                                        Theme::ERROR
                                    } else {
                                        Theme::WARNING
                                    }
                                } else {
                                    Theme::SUCCESS
                                };
                                ui.label(
                                    RichText::new(check_result.power_plan.label())
                                        .size(Theme::SIZE_CAPTION)
                                        .color(plan_color),
                                );
                                ui.end_row();
                            });
                    });

                ui.add_space(12.0);

                // High CPU Processes (if any)
                if !check_result.high_cpu_processes.is_empty() {
                    egui::Frame::none()
                        .fill(Theme::BG_CARD)
                        .stroke(egui::Stroke::new(1.0, Theme::BORDER))
                        .rounding(Theme::CARD_ROUNDING)
                        .inner_margin(12.0)
                        .show(ui, |ui| {
                            ui.set_min_width(500.0);

                            ui.label(
                                RichText::new("Active Processes")
                                    .size(Theme::SIZE_BODY)
                                    .strong()
                                    .color(Theme::TEXT_PRIMARY),
                            );
                            ui.add_space(8.0);

                            egui::Grid::new("processes_grid")
                                .num_columns(3)
                                .spacing([16.0, 4.0])
                                .striped(true)
                                .show(ui, |ui| {
                                    // Header
                                    ui.label(
                                        RichText::new("Process")
                                            .size(Theme::SIZE_CAPTION)
                                            .strong()
                                            .color(Theme::TEXT_SECONDARY),
                                    );
                                    ui.label(
                                        RichText::new("PID")
                                            .size(Theme::SIZE_CAPTION)
                                            .strong()
                                            .color(Theme::TEXT_SECONDARY),
                                    );
                                    ui.label(
                                        RichText::new("CPU")
                                            .size(Theme::SIZE_CAPTION)
                                            .strong()
                                            .color(Theme::TEXT_SECONDARY),
                                    );
                                    ui.end_row();

                                    for process in &check_result.high_cpu_processes {
                                        ui.label(
                                            RichText::new(&process.name)
                                                .size(Theme::SIZE_CAPTION)
                                                .color(Theme::TEXT_PRIMARY),
                                        );
                                        ui.label(
                                            RichText::new(format!("{}", process.pid))
                                                .size(Theme::SIZE_CAPTION)
                                                .color(Theme::TEXT_SECONDARY),
                                        );
                                        let cpu_color = if process.cpu_percent > 25.0 {
                                            Theme::ERROR
                                        } else if process.cpu_percent > 10.0 {
                                            Theme::WARNING
                                        } else {
                                            Theme::TEXT_PRIMARY
                                        };
                                        ui.label(
                                            RichText::new(format!("{:.1}%", process.cpu_percent))
                                                .size(Theme::SIZE_CAPTION)
                                                .color(cpu_color),
                                        );
                                        ui.end_row();
                                    }
                                });
                        });

                    ui.add_space(12.0);
                }

                // Warnings Section
                if !check_result.warnings.is_empty() {
                    egui::Frame::none()
                        .fill(Theme::BG_CARD)
                        .stroke(egui::Stroke::new(1.0, Theme::BORDER))
                        .rounding(Theme::CARD_ROUNDING)
                        .inner_margin(12.0)
                        .show(ui, |ui| {
                            ui.set_min_width(500.0);

                            ui.label(
                                RichText::new("Warnings")
                                    .size(Theme::SIZE_BODY)
                                    .strong()
                                    .color(Theme::TEXT_PRIMARY),
                            );
                            ui.add_space(8.0);

                            for warning in &check_result.warnings {
                                Self::show_warning(ui, warning);
                                ui.add_space(6.0);
                            }
                        });

                    ui.add_space(12.0);
                }

                // Action Buttons
                ui.horizontal(|ui| {
                    // Cancel button
                    let cancel_btn = egui::Button::new(
                        RichText::new("Cancel").size(Theme::SIZE_BODY),
                    )
                    .min_size(egui::vec2(80.0, 32.0))
                    .rounding(Theme::CARD_ROUNDING);

                    if ui.add(cancel_btn).clicked() {
                        action = PreCheckAction::Cancel;
                    }

                    ui.add_space(8.0);

                    // Recheck button
                    let recheck_btn = egui::Button::new(
                        RichText::new("Recheck").size(Theme::SIZE_BODY),
                    )
                    .min_size(egui::vec2(80.0, 32.0))
                    .rounding(Theme::CARD_ROUNDING);

                    if ui.add(recheck_btn).clicked() {
                        action = PreCheckAction::Recheck;
                    }

                    ui.add_space(8.0);

                    // Proceed button
                    let proceed_text = if check_result.ready_to_benchmark {
                        "Start Benchmark"
                    } else {
                        "Proceed Anyway"
                    };
                    let proceed_color = if check_result.ready_to_benchmark {
                        Theme::ACCENT
                    } else {
                        Theme::WARNING
                    };

                    let proceed_btn = egui::Button::new(
                        RichText::new(proceed_text)
                            .size(Theme::SIZE_BODY)
                            .color(egui::Color32::WHITE),
                    )
                    .min_size(egui::vec2(120.0, 32.0))
                    .fill(proceed_color)
                    .rounding(Theme::CARD_ROUNDING);

                    if ui.add(proceed_btn).clicked() {
                        action = PreCheckAction::Proceed;
                    }
                });

                ui.add_space(16.0);
            });
        });

        action
    }

    fn show_warning(ui: &mut Ui, warning: &crate::core::system_check::SystemWarning) {
        let (severity_color, bg_color) = match warning.severity {
            WarningSeverity::Critical => (Theme::ERROR, egui::Color32::from_rgb(254, 226, 226)),
            WarningSeverity::Warning => (Theme::WARNING, egui::Color32::from_rgb(254, 243, 199)),
            WarningSeverity::Info => (Theme::ACCENT, egui::Color32::from_rgb(224, 231, 255)),
        };

        egui::Frame::none()
            .fill(bg_color)
            .rounding(4.0)
            .inner_margin(8.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    // Severity badge
                    egui::Frame::none()
                        .fill(severity_color)
                        .rounding(2.0)
                        .inner_margin(egui::vec2(4.0, 2.0))
                        .show(ui, |ui| {
                            ui.label(
                                RichText::new(warning.severity.label())
                                    .size(10.0)
                                    .color(egui::Color32::WHITE)
                                    .strong(),
                            );
                        });

                    ui.add_space(8.0);

                    ui.label(
                        RichText::new(&warning.title)
                            .size(Theme::SIZE_CAPTION)
                            .strong()
                            .color(Theme::TEXT_PRIMARY),
                    );
                });

                ui.add_space(4.0);

                ui.label(
                    RichText::new(&warning.description)
                        .size(Theme::SIZE_CAPTION)
                        .color(Theme::TEXT_SECONDARY),
                );

                if let Some(ref remediation) = warning.remediation {
                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("Fix:")
                                .size(Theme::SIZE_CAPTION)
                                .strong()
                                .color(Theme::TEXT_SECONDARY),
                        );
                        ui.label(
                            RichText::new(remediation)
                                .size(Theme::SIZE_CAPTION)
                                .color(Theme::TEXT_PRIMARY),
                        );
                    });
                }
            });
    }
}
