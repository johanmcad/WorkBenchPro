use egui::{Align, Color32, Layout, RichText, Ui};

use crate::models::{BenchmarkRun, Rating, TestResult};
use crate::ui::Theme;

pub struct ComparisonView;

impl ComparisonView {
    /// Returns true if back was clicked
    pub fn show(ui: &mut Ui, run_a: &BenchmarkRun, run_b: &BenchmarkRun) -> bool {
        let mut back_clicked = false;

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.add_space(20.0);

                // Title
                ui.label(RichText::new("Comparison").size(28.0).strong().color(Theme::ACCENT));

                ui.add_space(10.0);

                // Machine names header
                ui.horizontal(|ui| {
                    ui.add_space(200.0);
                    ui.label(
                        RichText::new(&run_a.machine_name)
                            .size(14.0)
                            .strong()
                            .color(Color32::from_rgb(100, 180, 255)),
                    );
                    ui.label(
                        RichText::new(run_a.timestamp.format("%m/%d %H:%M").to_string())
                            .size(12.0)
                            .color(Theme::TEXT_SECONDARY),
                    );
                    ui.add_space(60.0);
                    ui.label(
                        RichText::new(&run_b.machine_name)
                            .size(14.0)
                            .strong()
                            .color(Color32::from_rgb(255, 180, 100)),
                    );
                    ui.label(
                        RichText::new(run_b.timestamp.format("%m/%d %H:%M").to_string())
                            .size(12.0)
                            .color(Theme::TEXT_SECONDARY),
                    );
                });

                ui.add_space(20.0);

                // Overall score comparison
                Self::show_score_comparison(
                    ui,
                    "Overall Score",
                    run_a.scores.overall,
                    run_a.scores.overall_max,
                    &run_a.scores.rating,
                    run_b.scores.overall,
                    run_b.scores.overall_max,
                    &run_b.scores.rating,
                );

                ui.add_space(20.0);

                // Category comparisons
                ui.label(RichText::new("Category Scores").size(18.0).strong());
                ui.add_space(10.0);

                Self::show_score_comparison(
                    ui,
                    "Project Operations",
                    run_a.scores.categories.project_operations.score,
                    run_a.scores.categories.project_operations.max_score,
                    &run_a.scores.categories.project_operations.rating,
                    run_b.scores.categories.project_operations.score,
                    run_b.scores.categories.project_operations.max_score,
                    &run_b.scores.categories.project_operations.rating,
                );

                Self::show_score_comparison(
                    ui,
                    "Build Performance",
                    run_a.scores.categories.build_performance.score,
                    run_a.scores.categories.build_performance.max_score,
                    &run_a.scores.categories.build_performance.rating,
                    run_b.scores.categories.build_performance.score,
                    run_b.scores.categories.build_performance.max_score,
                    &run_b.scores.categories.build_performance.rating,
                );

                Self::show_score_comparison(
                    ui,
                    "Responsiveness",
                    run_a.scores.categories.responsiveness.score,
                    run_a.scores.categories.responsiveness.max_score,
                    &run_a.scores.categories.responsiveness.rating,
                    run_b.scores.categories.responsiveness.score,
                    run_b.scores.categories.responsiveness.max_score,
                    &run_b.scores.categories.responsiveness.rating,
                );

                ui.add_space(20.0);

                // Detailed test comparisons
                ui.label(RichText::new("Detailed Comparison").size(18.0).strong());
                ui.add_space(10.0);

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

                ui.add_space(30.0);

                // Back button
                if ui
                    .button(RichText::new("Back to History").size(14.0))
                    .clicked()
                {
                    back_clicked = true;
                }

                ui.add_space(30.0);
            });
        });

        back_clicked
    }

    fn show_score_comparison(
        ui: &mut Ui,
        name: &str,
        score_a: u32,
        max_a: u32,
        rating_a: &Rating,
        score_b: u32,
        max_b: u32,
        rating_b: &Rating,
    ) {
        let pct_a = (score_a as f64 / max_a as f64) * 100.0;
        let pct_b = (score_b as f64 / max_b as f64) * 100.0;
        let diff = pct_a - pct_b;

        egui::Frame::none()
            .fill(Theme::BG_CARD)
            .stroke(egui::Stroke::new(1.0, Theme::BORDER))
            .rounding(8.0)
            .inner_margin(12.0)
            .show(ui, |ui| {
                ui.set_min_width(600.0);

                ui.horizontal(|ui| {
                    // Name column
                    ui.label(RichText::new(name).size(14.0).strong());

                    ui.add_space(20.0);

                    // Run A score
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new(format!("{} / {}", score_a, max_a))
                                .color(Color32::from_rgb(100, 180, 255)),
                        );
                        ui.label(
                            RichText::new(format!("{:.1}%", pct_a))
                                .color(Theme::rating_color(rating_a)),
                        );
                    });

                    ui.add_space(40.0);

                    // Difference indicator
                    let (diff_text, diff_color) = if diff.abs() < 0.5 {
                        ("=".to_string(), Theme::TEXT_SECONDARY)
                    } else if diff > 0.0 {
                        (format!("+{:.1}%", diff), Theme::SUCCESS)
                    } else {
                        (format!("{:.1}%", diff), Theme::ERROR)
                    };

                    ui.label(RichText::new(diff_text).size(16.0).strong().color(diff_color));

                    ui.add_space(40.0);

                    // Run B score
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new(format!("{} / {}", score_b, max_b))
                                .color(Color32::from_rgb(255, 180, 100)),
                        );
                        ui.label(
                            RichText::new(format!("{:.1}%", pct_b))
                                .color(Theme::rating_color(rating_b)),
                        );
                    });
                });
            });

        ui.add_space(8.0);
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
            .rounding(8.0)
            .inner_margin(16.0)
            .show(ui, |ui| {
                ui.set_min_width(700.0);

                ui.label(RichText::new(category_name).size(16.0).strong());
                ui.add_space(12.0);

                egui::Grid::new(format!("comparison_grid_{}", category_name))
                    .num_columns(5)
                    .spacing([15.0, 6.0])
                    .striped(true)
                    .show(ui, |ui| {
                        // Header
                        ui.label(RichText::new("Test").strong());
                        ui.label(RichText::new("Run A").strong().color(Color32::from_rgb(100, 180, 255)));
                        ui.label(RichText::new("Run B").strong().color(Color32::from_rgb(255, 180, 100)));
                        ui.label(RichText::new("Diff").strong());
                        ui.label(RichText::new("").strong());
                        ui.end_row();

                        // Match tests by ID
                        for result_a in results_a {
                            let result_b = results_b
                                .iter()
                                .find(|r| r.test_id == result_a.test_id);

                            ui.label(&result_a.name);

                            // Run A value
                            ui.label(format!("{:.2} {}", result_a.value, result_a.unit));

                            // Run B value
                            if let Some(rb) = result_b {
                                ui.label(format!("{:.2} {}", rb.value, rb.unit));

                                // Calculate difference (lower is better for most metrics)
                                let diff_pct = if result_a.value != 0.0 {
                                    ((rb.value - result_a.value) / result_a.value) * 100.0
                                } else {
                                    0.0
                                };

                                let (diff_text, diff_color) = if diff_pct.abs() < 1.0 {
                                    ("~".to_string(), Theme::TEXT_SECONDARY)
                                } else if diff_pct < 0.0 {
                                    // Lower is better for time-based metrics
                                    (format!("{:.0}%", diff_pct), Theme::SUCCESS)
                                } else {
                                    (format!("+{:.0}%", diff_pct), Theme::ERROR)
                                };

                                ui.label(RichText::new(diff_text).color(diff_color));

                                // Score comparison
                                let score_diff = result_a.score as i32 - rb.score as i32;
                                let score_color = if score_diff > 0 {
                                    Theme::SUCCESS
                                } else if score_diff < 0 {
                                    Theme::ERROR
                                } else {
                                    Theme::TEXT_SECONDARY
                                };
                                ui.label(
                                    RichText::new(format!(
                                        "{}/{} vs {}/{}",
                                        result_a.score, result_a.max_score,
                                        rb.score, rb.max_score
                                    ))
                                    .size(11.0)
                                    .color(score_color),
                                );
                            } else {
                                ui.label("-");
                                ui.label("-");
                                ui.label("-");
                            }

                            ui.end_row();
                        }

                        // Show tests that are only in run B
                        for result_b in results_b {
                            if !results_a.iter().any(|r| r.test_id == result_b.test_id) {
                                ui.label(&result_b.name);
                                ui.label("-");
                                ui.label(format!("{:.2} {}", result_b.value, result_b.unit));
                                ui.label("-");
                                ui.label(
                                    RichText::new(format!("{}/{}", result_b.score, result_b.max_score))
                                        .size(11.0),
                                );
                                ui.end_row();
                            }
                        }
                    });
            });

        ui.add_space(16.0);
    }
}
