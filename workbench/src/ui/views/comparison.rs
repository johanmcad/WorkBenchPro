use egui::{Align, Color32, Layout, RichText, Ui};

use crate::models::{BenchmarkRun, TestResult};
use crate::ui::Theme;

/// Comparison View - Side-by-side raw value comparison
/// - Machine info headers
/// - Category sections with test-by-test comparison
/// - Difference column with percentage
pub struct ComparisonView;

// Colors for the two runs being compared
const RUN_A_COLOR: Color32 = Color32::from_rgb(59, 130, 246);  // Blue
const RUN_B_COLOR: Color32 = Color32::from_rgb(249, 115, 22);  // Orange

impl ComparisonView {
    /// Returns true if back was clicked
    pub fn show(ui: &mut Ui, run_a: &BenchmarkRun, run_b: &BenchmarkRun) -> bool {
        let mut back_clicked = false;

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.add_space(12.0);

                // Title
                ui.label(
                    RichText::new("Comparison")
                        .size(Theme::SIZE_SECTION)
                        .strong()
                        .color(Theme::ACCENT),
                );

                ui.add_space(8.0);

                // Legend for the two runs
                ui.horizontal(|ui| {
                    // Run A
                    egui::Frame::none()
                        .fill(RUN_A_COLOR.linear_multiply(0.15))
                        .rounding(Theme::BADGE_ROUNDING)
                        .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new(&run_a.machine_name)
                                        .size(Theme::SIZE_CAPTION)
                                        .strong()
                                        .color(RUN_A_COLOR),
                                );
                                ui.label(
                                    RichText::new(run_a.timestamp.format("%m/%d %H:%M").to_string())
                                        .size(Theme::SIZE_CAPTION)
                                        .color(Theme::TEXT_SECONDARY),
                                );
                            });
                        });

                    ui.add_space(12.0);

                    ui.label(
                        RichText::new("vs")
                            .size(Theme::SIZE_CAPTION)
                            .color(Theme::TEXT_SECONDARY),
                    );

                    ui.add_space(12.0);

                    // Run B
                    egui::Frame::none()
                        .fill(RUN_B_COLOR.linear_multiply(0.15))
                        .rounding(Theme::BADGE_ROUNDING)
                        .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new(&run_b.machine_name)
                                        .size(Theme::SIZE_CAPTION)
                                        .strong()
                                        .color(RUN_B_COLOR),
                                );
                                ui.label(
                                    RichText::new(run_b.timestamp.format("%m/%d %H:%M").to_string())
                                        .size(Theme::SIZE_CAPTION)
                                        .color(Theme::TEXT_SECONDARY),
                                );
                            });
                        });
                });

                ui.add_space(12.0);

                // Summary cards
                Self::show_summary_cards(ui, run_a, run_b);

                ui.add_space(8.0);

                // Detailed Comparison Section
                ui.label(
                    RichText::new("Detailed Comparison")
                        .size(Theme::SIZE_CARD)
                        .strong()
                        .color(Theme::TEXT_PRIMARY),
                );
                ui.add_space(4.0);

                Self::show_test_comparison(
                    ui,
                    "Project Operations",
                    &run_a.results.project_operations,
                    &run_b.results.project_operations,
                );

                Self::show_test_comparison(
                    ui,
                    "Build Performance",
                    &run_a.results.build_performance,
                    &run_b.results.build_performance,
                );

                Self::show_test_comparison(
                    ui,
                    "Responsiveness",
                    &run_a.results.responsiveness,
                    &run_b.results.responsiveness,
                );

                ui.add_space(12.0);

                // Back Button
                let back_btn = egui::Button::new(
                    RichText::new("Back to History").size(Theme::SIZE_BODY),
                )
                .min_size(egui::vec2(100.0, 32.0))
                .rounding(Theme::CARD_ROUNDING);

                if ui.add(back_btn).clicked() {
                    back_clicked = true;
                }

                ui.add_space(12.0);
            });
        });

        back_clicked
    }

    fn show_summary_cards(ui: &mut Ui, run_a: &BenchmarkRun, run_b: &BenchmarkRun) {
        ui.horizontal(|ui| {
            // Run A summary
            egui::Frame::none()
                .fill(Theme::BG_CARD)
                .stroke(egui::Stroke::new(2.0, RUN_A_COLOR.linear_multiply(0.5)))
                .rounding(Theme::CARD_ROUNDING)
                .inner_margin(12.0)
                .show(ui, |ui| {
                    ui.set_min_width(220.0);
                    ui.with_layout(Layout::top_down(Align::Center), |ui| {
                        ui.label(
                            RichText::new(&run_a.machine_name)
                                .size(Theme::SIZE_BODY)
                                .strong()
                                .color(RUN_A_COLOR),
                        );
                        let total_a = run_a.results.project_operations.len()
                            + run_a.results.build_performance.len()
                            + run_a.results.responsiveness.len();
                        ui.label(
                            RichText::new(format!("{} tests", total_a))
                                .size(Theme::SIZE_SECTION)
                                .strong()
                                .color(Theme::TEXT_PRIMARY),
                        );
                        ui.label(
                            RichText::new(run_a.timestamp.format("%Y-%m-%d %H:%M").to_string())
                                .size(Theme::SIZE_CAPTION)
                                .color(Theme::TEXT_SECONDARY),
                        );
                    });
                });

            ui.add_space(16.0);

            // Run B summary
            egui::Frame::none()
                .fill(Theme::BG_CARD)
                .stroke(egui::Stroke::new(2.0, RUN_B_COLOR.linear_multiply(0.5)))
                .rounding(Theme::CARD_ROUNDING)
                .inner_margin(12.0)
                .show(ui, |ui| {
                    ui.set_min_width(220.0);
                    ui.with_layout(Layout::top_down(Align::Center), |ui| {
                        ui.label(
                            RichText::new(&run_b.machine_name)
                                .size(Theme::SIZE_BODY)
                                .strong()
                                .color(RUN_B_COLOR),
                        );
                        let total_b = run_b.results.project_operations.len()
                            + run_b.results.build_performance.len()
                            + run_b.results.responsiveness.len();
                        ui.label(
                            RichText::new(format!("{} tests", total_b))
                                .size(Theme::SIZE_SECTION)
                                .strong()
                                .color(Theme::TEXT_PRIMARY),
                        );
                        ui.label(
                            RichText::new(run_b.timestamp.format("%Y-%m-%d %H:%M").to_string())
                                .size(Theme::SIZE_CAPTION)
                                .color(Theme::TEXT_SECONDARY),
                        );
                    });
                });
        });
    }

    fn show_test_comparison(
        ui: &mut Ui,
        category_name: &str,
        results_a: &[TestResult],
        results_b: &[TestResult],
    ) {
        if results_a.is_empty() && results_b.is_empty() {
            return;
        }

        egui::Frame::none()
            .fill(Theme::BG_CARD)
            .stroke(egui::Stroke::new(1.0, Theme::BORDER))
            .rounding(Theme::CARD_ROUNDING)
            .inner_margin(8.0)
            .show(ui, |ui| {
                ui.set_min_width(620.0);

                egui::CollapsingHeader::new(
                    RichText::new(format!(
                        "{} ({} / {} tests)",
                        category_name,
                        results_a.len(),
                        results_b.len()
                    ))
                    .size(Theme::SIZE_BODY)
                    .strong()
                    .color(Theme::TEXT_PRIMARY),
                )
                .default_open(true)
                .show(ui, |ui| {
                    ui.add_space(4.0);

                    egui::Grid::new(format!("comparison_grid_{}", category_name))
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
                                RichText::new("Run A")
                                    .size(Theme::SIZE_CAPTION)
                                    .strong()
                                    .color(RUN_A_COLOR),
                            );
                            ui.label(
                                RichText::new("Run B")
                                    .size(Theme::SIZE_CAPTION)
                                    .strong()
                                    .color(RUN_B_COLOR),
                            );
                            ui.label(
                                RichText::new("Diff")
                                    .size(Theme::SIZE_CAPTION)
                                    .strong()
                                    .color(Theme::TEXT_SECONDARY),
                            );
                            ui.end_row();

                            // Match tests by ID
                            for result_a in results_a {
                                let result_b = results_b
                                    .iter()
                                    .find(|r| r.test_id == result_a.test_id);

                                ui.label(
                                    RichText::new(&result_a.name)
                                        .size(Theme::SIZE_CAPTION)
                                        .color(Theme::TEXT_PRIMARY),
                                );

                                // Run A value
                                let value_a_str = Self::format_value(result_a.value);
                                ui.label(
                                    RichText::new(format!("{} {}", value_a_str, result_a.unit))
                                        .size(Theme::SIZE_CAPTION)
                                        .color(Theme::TEXT_PRIMARY),
                                );

                                // Run B value & difference
                                if let Some(rb) = result_b {
                                    let value_b_str = Self::format_value(rb.value);
                                    ui.label(
                                        RichText::new(format!("{} {}", value_b_str, rb.unit))
                                            .size(Theme::SIZE_CAPTION)
                                            .color(Theme::TEXT_PRIMARY),
                                    );

                                    // Calculate difference
                                    // For most metrics: lower is better (time, latency)
                                    // For throughput metrics (MB/s, ops/sec): higher is better
                                    let higher_is_better = Self::is_higher_better(&result_a.unit);
                                    let diff_pct = if result_a.value != 0.0 {
                                        ((rb.value - result_a.value) / result_a.value) * 100.0
                                    } else {
                                        0.0
                                    };

                                    let diff_color = Theme::diff_color(diff_pct, higher_is_better);
                                    let diff_text = if diff_pct.abs() < 1.0 {
                                        "~".to_string()
                                    } else if diff_pct > 0.0 {
                                        format!("+{:.1}%", diff_pct)
                                    } else {
                                        format!("{:.1}%", diff_pct)
                                    };

                                    ui.label(
                                        RichText::new(diff_text)
                                            .size(Theme::SIZE_CAPTION)
                                            .color(diff_color),
                                    );
                                } else {
                                    ui.label(
                                        RichText::new("-")
                                            .size(Theme::SIZE_CAPTION)
                                            .color(Theme::TEXT_SECONDARY),
                                    );
                                    ui.label(
                                        RichText::new("-")
                                            .size(Theme::SIZE_CAPTION)
                                            .color(Theme::TEXT_SECONDARY),
                                    );
                                }

                                ui.end_row();
                            }

                            // Show tests that are only in run B
                            for result_b in results_b {
                                if !results_a.iter().any(|r| r.test_id == result_b.test_id) {
                                    ui.label(
                                        RichText::new(&result_b.name)
                                            .size(Theme::SIZE_CAPTION)
                                            .color(Theme::TEXT_PRIMARY),
                                    );
                                    ui.label(
                                        RichText::new("-")
                                            .size(Theme::SIZE_CAPTION)
                                            .color(Theme::TEXT_SECONDARY),
                                    );
                                    let value_b_str = Self::format_value(result_b.value);
                                    ui.label(
                                        RichText::new(format!("{} {}", value_b_str, result_b.unit))
                                            .size(Theme::SIZE_CAPTION)
                                            .color(Theme::TEXT_PRIMARY),
                                    );
                                    ui.label(
                                        RichText::new("-")
                                            .size(Theme::SIZE_CAPTION)
                                            .color(Theme::TEXT_SECONDARY),
                                    );
                                    ui.end_row();
                                }
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

    /// Determine if higher values are better for a given unit
    fn is_higher_better(unit: &str) -> bool {
        let unit_lower = unit.to_lowercase();
        // Throughput metrics - higher is better
        unit_lower.contains("/s")
            || unit_lower.contains("mb/s")
            || unit_lower.contains("ops")
            || unit_lower.contains("files")
            || unit_lower.contains("iops")
            || unit_lower.contains("gb/s")
    }
}
