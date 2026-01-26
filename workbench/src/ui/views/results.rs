use egui::{Align, Layout, RichText, Ui};

use crate::models::BenchmarkRun;
use crate::ui::widgets::{CategorySummaryCard, MachineInfoCard};
use crate::ui::Theme;

/// Actions from the results view for online features
pub enum ResultsAction {
    None,
    Back,
    Export,
    History,
    CompareOnline,
    CommunityComparison,
    Upload,
}

/// Results View - displays raw benchmark values without scores
/// Layout:
/// - Machine info header with timestamp
/// - Category summary cards
/// - Expandable detail sections with raw values
/// - Export/Compare buttons
pub struct ResultsView;

impl ResultsView {
    /// Returns (back_clicked, export_clicked)
    pub fn show(ui: &mut Ui, run: &BenchmarkRun) -> (bool, bool) {
        let mut back_clicked = false;
        let mut export_clicked = false;

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.add_space(12.0);

                // Title
                ui.label(
                    RichText::new("Benchmark Results")
                        .size(Theme::SIZE_SECTION)
                        .strong()
                        .color(Theme::ACCENT),
                );

                ui.add_space(8.0);

                // Machine Info Card
                let total_tests = run.results.project_operations.len()
                    + run.results.build_performance.len()
                    + run.results.responsiveness.len();

                ui.add(MachineInfoCard::new(
                    &run.machine_name,
                    &run.timestamp.format("%Y-%m-%d %H:%M").to_string(),
                    total_tests,
                ));

                ui.add_space(16.0);

                // Category Summary Cards
                ui.label(
                    RichText::new("Categories")
                        .size(Theme::SIZE_CARD)
                        .strong()
                        .color(Theme::TEXT_PRIMARY),
                );
                ui.add_space(8.0);

                ui.horizontal_wrapped(|ui| {
                    ui.add_space(8.0);

                    if !run.results.project_operations.is_empty() {
                        let summary = Self::get_category_summary(&run.results.project_operations);
                        ui.add(CategorySummaryCard::new(
                            "Project Operations",
                            run.results.project_operations.len(),
                            &summary,
                        ));
                        ui.add_space(8.0);
                    }

                    if !run.results.build_performance.is_empty() {
                        let summary = Self::get_category_summary(&run.results.build_performance);
                        ui.add(CategorySummaryCard::new(
                            "Build Performance",
                            run.results.build_performance.len(),
                            &summary,
                        ));
                        ui.add_space(8.0);
                    }

                    if !run.results.responsiveness.is_empty() {
                        let summary = Self::get_category_summary(&run.results.responsiveness);
                        ui.add(CategorySummaryCard::new(
                            "Responsiveness",
                            run.results.responsiveness.len(),
                            &summary,
                        ));
                    }
                });

                ui.add_space(16.0);

                // Detailed Results Section (expandable)
                ui.label(
                    RichText::new("Detailed Results")
                        .size(Theme::SIZE_CARD)
                        .strong()
                        .color(Theme::TEXT_PRIMARY),
                );
                ui.add_space(8.0);

                Self::show_category_details(ui, "Project Operations", &run.results.project_operations);
                Self::show_category_details(ui, "Build Performance", &run.results.build_performance);
                Self::show_category_details(ui, "Responsiveness", &run.results.responsiveness);

                ui.add_space(16.0);

                // Action Buttons
                ui.horizontal(|ui| {
                    let back_btn = egui::Button::new(
                        RichText::new("Back to Home").size(Theme::SIZE_BODY),
                    )
                    .min_size(egui::vec2(100.0, 32.0))
                    .rounding(Theme::CARD_ROUNDING);

                    if ui.add(back_btn).clicked() {
                        back_clicked = true;
                    }

                    ui.add_space(8.0);

                    let export_btn = egui::Button::new(
                        RichText::new("Export JSON").size(Theme::SIZE_BODY),
                    )
                    .min_size(egui::vec2(100.0, 32.0))
                    .rounding(Theme::CARD_ROUNDING);

                    if ui.add(export_btn).clicked() {
                        export_clicked = true;
                    }
                });

                ui.add_space(12.0);
            });
        });

        (back_clicked, export_clicked)
    }

    /// Returns ResultsAction for the main results view with online features
    pub fn show_with_save(ui: &mut Ui, run: &BenchmarkRun) -> ResultsAction {
        let mut action = ResultsAction::None;

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.add_space(12.0);

                // Title
                ui.label(
                    RichText::new("Benchmark Results")
                        .size(Theme::SIZE_SECTION)
                        .strong()
                        .color(Theme::ACCENT),
                );

                // Saved indicator
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Saved").size(Theme::SIZE_CAPTION).color(Theme::SUCCESS));
                    if run.uploaded_at.is_some() {
                        ui.label(RichText::new(" | ").size(Theme::SIZE_CAPTION).color(Theme::TEXT_SECONDARY));
                        ui.label(RichText::new("Uploaded").size(Theme::SIZE_CAPTION).color(Theme::ACCENT));
                    }
                });

                ui.add_space(8.0);

                // Machine Info Card
                let total_tests = run.results.project_operations.len()
                    + run.results.build_performance.len()
                    + run.results.responsiveness.len();

                ui.add(MachineInfoCard::new(
                    &run.machine_name,
                    &run.timestamp.format("%Y-%m-%d %H:%M").to_string(),
                    total_tests,
                ));

                ui.add_space(16.0);

                // Category Summary Cards
                ui.label(
                    RichText::new("Categories")
                        .size(Theme::SIZE_CARD)
                        .strong()
                        .color(Theme::TEXT_PRIMARY),
                );
                ui.add_space(8.0);

                ui.horizontal_wrapped(|ui| {
                    ui.add_space(8.0);

                    if !run.results.project_operations.is_empty() {
                        let summary = Self::get_category_summary(&run.results.project_operations);
                        ui.add(CategorySummaryCard::new(
                            "Project Operations",
                            run.results.project_operations.len(),
                            &summary,
                        ));
                        ui.add_space(8.0);
                    }

                    if !run.results.build_performance.is_empty() {
                        let summary = Self::get_category_summary(&run.results.build_performance);
                        ui.add(CategorySummaryCard::new(
                            "Build Performance",
                            run.results.build_performance.len(),
                            &summary,
                        ));
                        ui.add_space(8.0);
                    }

                    if !run.results.responsiveness.is_empty() {
                        let summary = Self::get_category_summary(&run.results.responsiveness);
                        ui.add(CategorySummaryCard::new(
                            "Responsiveness",
                            run.results.responsiveness.len(),
                            &summary,
                        ));
                    }
                });

                ui.add_space(16.0);

                // Detailed Results Section
                ui.label(
                    RichText::new("Detailed Results")
                        .size(Theme::SIZE_CARD)
                        .strong()
                        .color(Theme::TEXT_PRIMARY),
                );
                ui.add_space(8.0);

                Self::show_category_details(ui, "Project Operations", &run.results.project_operations);
                Self::show_category_details(ui, "Build Performance", &run.results.build_performance);
                Self::show_category_details(ui, "Responsiveness", &run.results.responsiveness);

                ui.add_space(16.0);

                // Action Buttons - Row 1: Navigation
                ui.horizontal(|ui| {
                    let back_btn = egui::Button::new(
                        RichText::new("Back to Home").size(Theme::SIZE_BODY),
                    )
                    .min_size(egui::vec2(100.0, 32.0))
                    .rounding(Theme::CARD_ROUNDING);

                    if ui.add(back_btn).clicked() {
                        action = ResultsAction::Back;
                    }

                    ui.add_space(6.0);

                    let export_btn = egui::Button::new(
                        RichText::new("Export JSON").size(Theme::SIZE_BODY),
                    )
                    .min_size(egui::vec2(100.0, 32.0))
                    .rounding(Theme::CARD_ROUNDING);

                    if ui.add(export_btn).clicked() {
                        action = ResultsAction::Export;
                    }

                    ui.add_space(6.0);

                    let history_btn = egui::Button::new(
                        RichText::new("View History").size(Theme::SIZE_BODY),
                    )
                    .min_size(egui::vec2(100.0, 32.0))
                    .rounding(Theme::CARD_ROUNDING);

                    if ui.add(history_btn).clicked() {
                        action = ResultsAction::History;
                    }
                });

                ui.add_space(8.0);

                // Action Buttons - Row 2: Online features
                ui.horizontal(|ui| {
                    let compare_btn = egui::Button::new(
                        RichText::new("Compare Online")
                            .size(Theme::SIZE_BODY)
                            .color(egui::Color32::WHITE),
                    )
                    .min_size(egui::vec2(120.0, 32.0))
                    .fill(Theme::ACCENT)
                    .rounding(Theme::CARD_ROUNDING);

                    if ui.add(compare_btn).clicked() {
                        action = ResultsAction::CompareOnline;
                    }

                    ui.add_space(6.0);

                    // Only show upload if not already uploaded
                    if run.uploaded_at.is_none() {
                        let upload_btn = egui::Button::new(
                            RichText::new("Upload to Community").size(Theme::SIZE_BODY),
                        )
                        .min_size(egui::vec2(140.0, 32.0))
                        .rounding(Theme::CARD_ROUNDING);

                        if ui.add(upload_btn).clicked() {
                            action = ResultsAction::Upload;
                        }
                    } else {
                        // Show Community Stats button for uploaded runs
                        let stats_btn = egui::Button::new(
                            RichText::new("Community Stats")
                                .size(Theme::SIZE_BODY)
                                .color(egui::Color32::WHITE),
                        )
                        .min_size(egui::vec2(120.0, 32.0))
                        .fill(Theme::SUCCESS)
                        .rounding(Theme::CARD_ROUNDING);

                        if ui.add(stats_btn).clicked() {
                            action = ResultsAction::CommunityComparison;
                        }
                    }
                });

                ui.add_space(12.0);
            });
        });

        action
    }

    fn get_category_summary(results: &[crate::models::TestResult]) -> String {
        if results.is_empty() {
            return "No tests".to_string();
        }
        // Just show "completed" status
        "completed".to_string()
    }

    fn show_category_details(ui: &mut Ui, category_name: &str, results: &[crate::models::TestResult]) {
        if results.is_empty() {
            return;
        }

        egui::Frame::none()
            .fill(Theme::BG_CARD)
            .stroke(egui::Stroke::new(1.0, Theme::BORDER))
            .rounding(Theme::CARD_ROUNDING)
            .inner_margin(8.0)
            .show(ui, |ui| {
                ui.set_min_width(600.0);

                // Collapsible header
                egui::CollapsingHeader::new(
                    RichText::new(format!("{} ({} tests)", category_name, results.len()))
                        .size(Theme::SIZE_BODY)
                        .strong()
                        .color(Theme::TEXT_PRIMARY),
                )
                .default_open(false)
                .show(ui, |ui| {
                    ui.add_space(4.0);

                    egui::Grid::new(format!("results_grid_{}", category_name))
                        .num_columns(4)
                        .spacing([16.0, 4.0])
                        .striped(true)
                        .show(ui, |ui| {
                            // Header
                            ui.label(
                                RichText::new("Test")
                                    .size(Theme::SIZE_CAPTION)
                                    .strong()
                                    .color(Theme::TEXT_SECONDARY),
                            );
                            ui.label(
                                RichText::new("Value")
                                    .size(Theme::SIZE_CAPTION)
                                    .strong()
                                    .color(Theme::TEXT_SECONDARY),
                            );
                            ui.label(
                                RichText::new("Min")
                                    .size(Theme::SIZE_CAPTION)
                                    .strong()
                                    .color(Theme::TEXT_SECONDARY),
                            );
                            ui.label(
                                RichText::new("Max")
                                    .size(Theme::SIZE_CAPTION)
                                    .strong()
                                    .color(Theme::TEXT_SECONDARY),
                            );
                            ui.end_row();

                            for result in results {
                                // Test name
                                ui.label(
                                    RichText::new(&result.name)
                                        .size(Theme::SIZE_CAPTION)
                                        .color(Theme::TEXT_PRIMARY),
                                );

                                // Primary value with unit
                                let value_str = Self::format_value(result.value);
                                ui.label(
                                    RichText::new(format!("{} {}", value_str, result.unit))
                                        .size(Theme::SIZE_CAPTION)
                                        .color(Theme::ACCENT)
                                        .strong(),
                                );

                                // Min value
                                let min_str = Self::format_value(result.details.min);
                                ui.label(
                                    RichText::new(min_str)
                                        .size(Theme::SIZE_CAPTION)
                                        .color(Theme::TEXT_SECONDARY),
                                );

                                // Max value
                                let max_str = Self::format_value(result.details.max);
                                ui.label(
                                    RichText::new(max_str)
                                        .size(Theme::SIZE_CAPTION)
                                        .color(Theme::TEXT_SECONDARY),
                                );
                                ui.end_row();
                            }
                        });
                });
            });

        ui.add_space(4.0);
    }

    fn format_value(value: f64) -> String {
        if value >= 10000.0 {
            format!("{:.0}", value)
        } else if value >= 100.0 {
            format!("{:.1}", value)
        } else if value >= 1.0 {
            format!("{:.2}", value)
        } else {
            format!("{:.3}", value)
        }
    }
}
