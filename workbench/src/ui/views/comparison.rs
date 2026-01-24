use egui::{Align, Color32, Layout, RichText, Ui};

use crate::models::{BenchmarkRun, Rating, TestResult};
use crate::ui::widgets::ProgressBar;
use crate::ui::Theme;

/// Comparison View matching 05-ui-design.md spec:
/// - Side-by-side score cards
/// - Category bar comparison
/// - Difference column with multipliers
/// - Key metrics table
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
                ui.add_space(24.0);

                // Title
                ui.label(
                    RichText::new("Comparison")
                        .size(Theme::SIZE_TITLE)
                        .strong()
                        .color(Theme::ACCENT),
                );

                ui.add_space(16.0);

                // Legend for the two runs
                ui.horizontal(|ui| {
                    // Run A
                    egui::Frame::none()
                        .fill(RUN_A_COLOR.linear_multiply(0.15))
                        .rounding(Theme::BADGE_ROUNDING)
                        .inner_margin(egui::Margin::symmetric(12.0, 6.0))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new(&run_a.machine_name)
                                        .size(Theme::SIZE_BODY)
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

                    ui.add_space(24.0);

                    ui.label(
                        RichText::new("vs")
                            .size(Theme::SIZE_BODY)
                            .color(Theme::TEXT_SECONDARY),
                    );

                    ui.add_space(24.0);

                    // Run B
                    egui::Frame::none()
                        .fill(RUN_B_COLOR.linear_multiply(0.15))
                        .rounding(Theme::BADGE_ROUNDING)
                        .inner_margin(egui::Margin::symmetric(12.0, 6.0))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new(&run_b.machine_name)
                                        .size(Theme::SIZE_BODY)
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

                ui.add_space(32.0);

                // Overall Score Comparison
                Self::show_score_comparison_card(
                    ui,
                    "Overall Score",
                    run_a.scores.overall,
                    run_a.scores.overall_max,
                    &run_a.scores.rating,
                    run_b.scores.overall,
                    run_b.scores.overall_max,
                    &run_b.scores.rating,
                    true,
                );

                ui.add_space(24.0);

                // Category Scores Section
                ui.label(
                    RichText::new("Category Scores")
                        .size(Theme::SIZE_SECTION)
                        .strong()
                        .color(Theme::TEXT_PRIMARY),
                );
                ui.add_space(16.0);

                Self::show_score_comparison_card(
                    ui,
                    "Project Operations",
                    run_a.scores.categories.project_operations.score,
                    run_a.scores.categories.project_operations.max_score,
                    &run_a.scores.categories.project_operations.rating,
                    run_b.scores.categories.project_operations.score,
                    run_b.scores.categories.project_operations.max_score,
                    &run_b.scores.categories.project_operations.rating,
                    false,
                );

                Self::show_score_comparison_card(
                    ui,
                    "Build Performance",
                    run_a.scores.categories.build_performance.score,
                    run_a.scores.categories.build_performance.max_score,
                    &run_a.scores.categories.build_performance.rating,
                    run_b.scores.categories.build_performance.score,
                    run_b.scores.categories.build_performance.max_score,
                    &run_b.scores.categories.build_performance.rating,
                    false,
                );

                Self::show_score_comparison_card(
                    ui,
                    "Responsiveness",
                    run_a.scores.categories.responsiveness.score,
                    run_a.scores.categories.responsiveness.max_score,
                    &run_a.scores.categories.responsiveness.rating,
                    run_b.scores.categories.responsiveness.score,
                    run_b.scores.categories.responsiveness.max_score,
                    &run_b.scores.categories.responsiveness.rating,
                    false,
                );

                ui.add_space(24.0);

                // Detailed Comparison Section
                ui.label(
                    RichText::new("Detailed Comparison")
                        .size(Theme::SIZE_SECTION)
                        .strong()
                        .color(Theme::TEXT_PRIMARY),
                );
                ui.add_space(16.0);

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

                ui.add_space(32.0);

                // Back Button
                let back_btn = egui::Button::new(
                    RichText::new("Back to History").size(Theme::SIZE_BODY),
                )
                .min_size(egui::vec2(140.0, 40.0))
                .rounding(Theme::CARD_ROUNDING);

                if ui.add(back_btn).clicked() {
                    back_clicked = true;
                }

                ui.add_space(32.0);
            });
        });

        back_clicked
    }

    fn show_score_comparison_card(
        ui: &mut Ui,
        name: &str,
        score_a: u32,
        max_a: u32,
        rating_a: &Rating,
        score_b: u32,
        max_b: u32,
        rating_b: &Rating,
        is_overall: bool,
    ) {
        let pct_a = (score_a as f64 / max_a as f64) * 100.0;
        let pct_b = (score_b as f64 / max_b as f64) * 100.0;
        let diff = pct_a - pct_b;

        let card_width = if is_overall { 700.0 } else { 650.0 };

        egui::Frame::none()
            .fill(Theme::BG_CARD)
            .stroke(egui::Stroke::new(1.0, Theme::BORDER))
            .rounding(Theme::CARD_ROUNDING)
            .inner_margin(16.0)
            .show(ui, |ui| {
                ui.set_min_width(card_width);

                // Category name
                ui.label(
                    RichText::new(name)
                        .size(if is_overall { Theme::SIZE_SECTION } else { Theme::SIZE_CARD })
                        .strong()
                        .color(Theme::TEXT_PRIMARY),
                );

                ui.add_space(12.0);

                // Side-by-side comparison bars
                ui.horizontal(|ui| {
                    // Run A side
                    ui.vertical(|ui| {
                        ui.set_width(250.0);
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(format!("{}", score_a))
                                    .size(Theme::SIZE_SCORE)
                                    .strong()
                                    .color(RUN_A_COLOR),
                            );
                            ui.label(
                                RichText::new(format!("/ {}", max_a))
                                    .size(Theme::SIZE_BODY)
                                    .color(Theme::TEXT_SECONDARY),
                            );
                        });
                        ui.add(
                            ProgressBar::new(pct_a as f32 / 100.0)
                                .rating(*rating_a)
                                .width(220.0),
                        );
                        ui.label(
                            RichText::new(format!("{:.1}%", pct_a))
                                .size(Theme::SIZE_CAPTION)
                                .color(Theme::rating_color(rating_a)),
                        );
                    });

                    ui.add_space(20.0);

                    // Difference indicator (center)
                    ui.vertical(|ui| {
                        ui.set_width(80.0);
                        ui.add_space(8.0);

                        let (diff_text, diff_color) = if diff.abs() < 0.5 {
                            ("=".to_string(), Theme::TEXT_SECONDARY)
                        } else if diff > 0.0 {
                            (format!("+{:.1}%", diff), Theme::SUCCESS)
                        } else {
                            (format!("{:.1}%", diff), Theme::ERROR)
                        };

                        ui.with_layout(Layout::top_down(Align::Center), |ui| {
                            egui::Frame::none()
                                .fill(if diff.abs() < 0.5 {
                                    Theme::BORDER
                                } else if diff > 0.0 {
                                    Theme::SUCCESS.linear_multiply(0.15)
                                } else {
                                    Theme::ERROR.linear_multiply(0.15)
                                })
                                .rounding(Theme::BADGE_ROUNDING)
                                .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                                .show(ui, |ui| {
                                    ui.label(
                                        RichText::new(diff_text)
                                            .size(Theme::SIZE_CARD)
                                            .strong()
                                            .color(diff_color),
                                    );
                                });
                        });
                    });

                    ui.add_space(20.0);

                    // Run B side
                    ui.vertical(|ui| {
                        ui.set_width(250.0);
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(format!("{}", score_b))
                                    .size(Theme::SIZE_SCORE)
                                    .strong()
                                    .color(RUN_B_COLOR),
                            );
                            ui.label(
                                RichText::new(format!("/ {}", max_b))
                                    .size(Theme::SIZE_BODY)
                                    .color(Theme::TEXT_SECONDARY),
                            );
                        });
                        ui.add(
                            ProgressBar::new(pct_b as f32 / 100.0)
                                .rating(*rating_b)
                                .width(220.0),
                        );
                        ui.label(
                            RichText::new(format!("{:.1}%", pct_b))
                                .size(Theme::SIZE_CAPTION)
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
            .rounding(Theme::CARD_ROUNDING)
            .inner_margin(16.0)
            .show(ui, |ui| {
                ui.set_min_width(750.0);

                egui::CollapsingHeader::new(
                    RichText::new(category_name)
                        .size(Theme::SIZE_CARD)
                        .strong()
                        .color(Theme::TEXT_PRIMARY),
                )
                .default_open(false)
                .show(ui, |ui| {
                    ui.add_space(8.0);

                    egui::Grid::new(format!("comparison_grid_{}", category_name))
                        .num_columns(5)
                        .spacing([16.0, 6.0])
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
                            ui.label(
                                RichText::new("Scores")
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
                                        .size(Theme::SIZE_BODY)
                                        .color(Theme::TEXT_PRIMARY),
                                );

                                // Run A value
                                ui.label(
                                    RichText::new(format!("{:.2} {}", result_a.value, result_a.unit))
                                        .size(Theme::SIZE_BODY)
                                        .color(Theme::TEXT_PRIMARY),
                                );

                                // Run B value & difference
                                if let Some(rb) = result_b {
                                    ui.label(
                                        RichText::new(format!("{:.2} {}", rb.value, rb.unit))
                                            .size(Theme::SIZE_BODY)
                                            .color(Theme::TEXT_PRIMARY),
                                    );

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

                                    ui.label(RichText::new(diff_text).size(Theme::SIZE_BODY).color(diff_color));

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
                                        .size(Theme::SIZE_CAPTION)
                                        .color(score_color),
                                    );
                                } else {
                                    ui.label(RichText::new("-").color(Theme::TEXT_SECONDARY));
                                    ui.label(RichText::new("-").color(Theme::TEXT_SECONDARY));
                                    ui.label(RichText::new("-").color(Theme::TEXT_SECONDARY));
                                }

                                ui.end_row();
                            }

                            // Show tests that are only in run B
                            for result_b in results_b {
                                if !results_a.iter().any(|r| r.test_id == result_b.test_id) {
                                    ui.label(
                                        RichText::new(&result_b.name)
                                            .size(Theme::SIZE_BODY)
                                            .color(Theme::TEXT_PRIMARY),
                                    );
                                    ui.label(RichText::new("-").color(Theme::TEXT_SECONDARY));
                                    ui.label(
                                        RichText::new(format!("{:.2} {}", result_b.value, result_b.unit))
                                            .size(Theme::SIZE_BODY)
                                            .color(Theme::TEXT_PRIMARY),
                                    );
                                    ui.label(RichText::new("-").color(Theme::TEXT_SECONDARY));
                                    ui.label(
                                        RichText::new(format!("{}/{}", result_b.score, result_b.max_score))
                                            .size(Theme::SIZE_CAPTION)
                                            .color(Theme::TEXT_SECONDARY),
                                    );
                                    ui.end_row();
                                }
                            }
                        });
                });
            });

        ui.add_space(12.0);
    }
}
