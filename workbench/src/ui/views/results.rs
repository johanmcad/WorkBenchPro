use egui::{Align, Layout, RichText, Ui};

use crate::models::BenchmarkRun;
use crate::ui::widgets::{LargeScoreCard, ScoreCard};
use crate::ui::Theme;

/// Results View matching 05-ui-design.md spec:
/// - Overall score card (large)
/// - 4 category cards in grid
/// - Expandable detail sections
/// - Export/Compare buttons
pub struct ResultsView;

impl ResultsView {
    /// Returns (back_clicked, export_clicked)
    pub fn show(ui: &mut Ui, run: &BenchmarkRun) -> (bool, bool) {
        let mut back_clicked = false;
        let mut export_clicked = false;

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.add_space(24.0);

                // Title
                ui.label(
                    RichText::new("Benchmark Results")
                        .size(Theme::SIZE_TITLE)
                        .strong()
                        .color(Theme::ACCENT),
                );
                ui.add_space(4.0);
                ui.label(
                    RichText::new(format!(
                        "{} - {}",
                        run.machine_name,
                        run.timestamp.format("%Y-%m-%d %H:%M")
                    ))
                    .size(Theme::SIZE_BODY)
                    .color(Theme::TEXT_SECONDARY),
                );

                ui.add_space(32.0);

                // Overall Score Card (large, centered)
                ui.add(LargeScoreCard::new(
                    "Overall Score",
                    run.scores.overall,
                    run.scores.overall_max,
                    run.scores.rating,
                ));

                ui.add_space(32.0);

                // Category Scores Section
                ui.label(
                    RichText::new("Category Scores")
                        .size(Theme::SIZE_SECTION)
                        .strong()
                        .color(Theme::TEXT_PRIMARY),
                );
                ui.add_space(16.0);

                // 4 category cards in a grid (2x2 or horizontal wrap)
                ui.horizontal_wrapped(|ui| {
                    ui.add_space(20.0);
                    ui.add(ScoreCard::new(
                        "Project Operations",
                        run.scores.categories.project_operations.score,
                        run.scores.categories.project_operations.max_score,
                        run.scores.categories.project_operations.rating,
                    ));

                    ui.add_space(16.0);

                    ui.add(ScoreCard::new(
                        "Build Performance",
                        run.scores.categories.build_performance.score,
                        run.scores.categories.build_performance.max_score,
                        run.scores.categories.build_performance.rating,
                    ));

                    ui.add_space(16.0);

                    ui.add(ScoreCard::new(
                        "Responsiveness",
                        run.scores.categories.responsiveness.score,
                        run.scores.categories.responsiveness.max_score,
                        run.scores.categories.responsiveness.rating,
                    ));

                    if let Some(graphics) = &run.scores.categories.graphics {
                        ui.add_space(16.0);
                        ui.add(ScoreCard::new(
                            "Graphics",
                            graphics.score,
                            graphics.max_score,
                            graphics.rating,
                        ));
                    }
                });

                ui.add_space(32.0);

                // Detailed Results Section (expandable)
                ui.label(
                    RichText::new("Detailed Results")
                        .size(Theme::SIZE_SECTION)
                        .strong()
                        .color(Theme::TEXT_PRIMARY),
                );
                ui.add_space(16.0);

                Self::show_category_details(ui, "Project Operations", &run.results.project_operations);
                Self::show_category_details(ui, "Build Performance", &run.results.build_performance);
                Self::show_category_details(ui, "Responsiveness", &run.results.responsiveness);

                if let Some(graphics) = &run.results.graphics {
                    Self::show_category_details(ui, "Graphics", graphics);
                }

                ui.add_space(32.0);

                // Action Buttons
                ui.horizontal(|ui| {
                    let back_btn = egui::Button::new(
                        RichText::new("Back to Home").size(Theme::SIZE_BODY),
                    )
                    .min_size(egui::vec2(120.0, 40.0))
                    .rounding(Theme::CARD_ROUNDING);

                    if ui.add(back_btn).clicked() {
                        back_clicked = true;
                    }

                    ui.add_space(16.0);

                    let export_btn = egui::Button::new(
                        RichText::new("Export JSON").size(Theme::SIZE_BODY),
                    )
                    .min_size(egui::vec2(120.0, 40.0))
                    .rounding(Theme::CARD_ROUNDING);

                    if ui.add(export_btn).clicked() {
                        export_clicked = true;
                    }
                });

                ui.add_space(32.0);
            });
        });

        (back_clicked, export_clicked)
    }

    /// Returns (back_clicked, export_clicked, history_clicked)
    pub fn show_with_save(ui: &mut Ui, run: &BenchmarkRun) -> (bool, bool, bool) {
        let mut back_clicked = false;
        let mut export_clicked = false;
        let mut history_clicked = false;

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.add_space(24.0);

                // Title
                ui.label(
                    RichText::new("Benchmark Results")
                        .size(Theme::SIZE_TITLE)
                        .strong()
                        .color(Theme::ACCENT),
                );
                ui.add_space(4.0);
                ui.label(
                    RichText::new(format!(
                        "{} - {}",
                        run.machine_name,
                        run.timestamp.format("%Y-%m-%d %H:%M")
                    ))
                    .size(Theme::SIZE_BODY)
                    .color(Theme::TEXT_SECONDARY),
                );

                ui.add_space(8.0);

                // Saved indicator
                ui.horizontal(|ui| {
                    ui.label(RichText::new("✓").color(Theme::SUCCESS));
                    ui.label(
                        RichText::new("Results saved to history")
                            .size(Theme::SIZE_CAPTION)
                            .color(Theme::SUCCESS)
                            .italics(),
                    );
                });

                ui.add_space(24.0);

                // Overall Score Card (large, centered)
                ui.add(LargeScoreCard::new(
                    "Overall Score",
                    run.scores.overall,
                    run.scores.overall_max,
                    run.scores.rating,
                ));

                ui.add_space(32.0);

                // Category Scores Section
                ui.label(
                    RichText::new("Category Scores")
                        .size(Theme::SIZE_SECTION)
                        .strong()
                        .color(Theme::TEXT_PRIMARY),
                );
                ui.add_space(16.0);

                // 4 category cards in a grid
                ui.horizontal_wrapped(|ui| {
                    ui.add_space(20.0);
                    ui.add(ScoreCard::new(
                        "Project Operations",
                        run.scores.categories.project_operations.score,
                        run.scores.categories.project_operations.max_score,
                        run.scores.categories.project_operations.rating,
                    ));

                    ui.add_space(16.0);

                    ui.add(ScoreCard::new(
                        "Build Performance",
                        run.scores.categories.build_performance.score,
                        run.scores.categories.build_performance.max_score,
                        run.scores.categories.build_performance.rating,
                    ));

                    ui.add_space(16.0);

                    ui.add(ScoreCard::new(
                        "Responsiveness",
                        run.scores.categories.responsiveness.score,
                        run.scores.categories.responsiveness.max_score,
                        run.scores.categories.responsiveness.rating,
                    ));

                    if let Some(graphics) = &run.scores.categories.graphics {
                        ui.add_space(16.0);
                        ui.add(ScoreCard::new(
                            "Graphics",
                            graphics.score,
                            graphics.max_score,
                            graphics.rating,
                        ));
                    }
                });

                ui.add_space(32.0);

                // Detailed Results Section
                ui.label(
                    RichText::new("Detailed Results")
                        .size(Theme::SIZE_SECTION)
                        .strong()
                        .color(Theme::TEXT_PRIMARY),
                );
                ui.add_space(16.0);

                Self::show_category_details(ui, "Project Operations", &run.results.project_operations);
                Self::show_category_details(ui, "Build Performance", &run.results.build_performance);
                Self::show_category_details(ui, "Responsiveness", &run.results.responsiveness);

                if let Some(graphics) = &run.results.graphics {
                    Self::show_category_details(ui, "Graphics", graphics);
                }

                ui.add_space(32.0);

                // Action Buttons
                ui.horizontal(|ui| {
                    let back_btn = egui::Button::new(
                        RichText::new("Back to Home").size(Theme::SIZE_BODY),
                    )
                    .min_size(egui::vec2(120.0, 40.0))
                    .rounding(Theme::CARD_ROUNDING);

                    if ui.add(back_btn).clicked() {
                        back_clicked = true;
                    }

                    ui.add_space(12.0);

                    let export_btn = egui::Button::new(
                        RichText::new("Export JSON").size(Theme::SIZE_BODY),
                    )
                    .min_size(egui::vec2(120.0, 40.0))
                    .rounding(Theme::CARD_ROUNDING);

                    if ui.add(export_btn).clicked() {
                        export_clicked = true;
                    }

                    ui.add_space(12.0);

                    let history_btn = egui::Button::new(
                        RichText::new("View History").size(Theme::SIZE_BODY),
                    )
                    .min_size(egui::vec2(120.0, 40.0))
                    .rounding(Theme::CARD_ROUNDING);

                    if ui.add(history_btn).clicked() {
                        history_clicked = true;
                    }
                });

                ui.add_space(32.0);
            });
        });

        (back_clicked, export_clicked, history_clicked)
    }

    fn show_category_details(ui: &mut Ui, category_name: &str, results: &[crate::models::TestResult]) {
        if results.is_empty() {
            return;
        }

        egui::Frame::none()
            .fill(Theme::BG_CARD)
            .stroke(egui::Stroke::new(1.0, Theme::BORDER))
            .rounding(Theme::CARD_ROUNDING)
            .inner_margin(16.0)
            .show(ui, |ui| {
                ui.set_min_width(700.0);

                // Collapsible header
                egui::CollapsingHeader::new(
                    RichText::new(category_name)
                        .size(Theme::SIZE_CARD)
                        .strong()
                        .color(Theme::TEXT_PRIMARY),
                )
                .default_open(false)
                .show(ui, |ui| {
                    ui.add_space(8.0);

                    egui::Grid::new(format!("results_grid_{}", category_name))
                        .num_columns(4)
                        .spacing([24.0, 8.0])
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
                                RichText::new("Score")
                                    .size(Theme::SIZE_CAPTION)
                                    .strong()
                                    .color(Theme::TEXT_SECONDARY),
                            );
                            ui.label(
                                RichText::new("Rating")
                                    .size(Theme::SIZE_CAPTION)
                                    .strong()
                                    .color(Theme::TEXT_SECONDARY),
                            );
                            ui.end_row();

                            for result in results {
                                ui.label(
                                    RichText::new(&result.name)
                                        .size(Theme::SIZE_BODY)
                                        .color(Theme::TEXT_PRIMARY),
                                );
                                ui.label(
                                    RichText::new(format!("{:.2} {}", result.value, result.unit))
                                        .size(Theme::SIZE_BODY)
                                        .color(Theme::TEXT_PRIMARY),
                                );
                                ui.label(
                                    RichText::new(format!("{} / {}", result.score, result.max_score))
                                        .size(Theme::SIZE_BODY)
                                        .color(Theme::TEXT_PRIMARY),
                                );

                                let percentage = result.score_percentage();
                                let rating = crate::models::Rating::from_percentage(percentage);

                                // Rating badge
                                egui::Frame::none()
                                    .fill(Theme::rating_bg_color(&rating))
                                    .rounding(Theme::BADGE_ROUNDING)
                                    .inner_margin(egui::Margin::symmetric(6.0, 2.0))
                                    .show(ui, |ui| {
                                        ui.label(
                                            RichText::new(format!("{:.0}%", percentage))
                                                .size(Theme::SIZE_CAPTION)
                                                .color(Theme::rating_color(&rating)),
                                        );
                                    });
                                ui.end_row();
                            }
                        });
                });
            });

        ui.add_space(12.0);
    }
}
