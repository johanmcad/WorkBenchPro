use egui::{Align, Layout, RichText, Ui};

use crate::models::BenchmarkRun;
use crate::ui::widgets::{LargeScoreCard, ScoreCard};
use crate::ui::Theme;

pub struct ResultsView;

impl ResultsView {
    /// Returns (back_clicked, export_clicked)
    pub fn show(ui: &mut Ui, run: &BenchmarkRun) -> (bool, bool) {
        let mut back_clicked = false;
        let mut export_clicked = false;

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.add_space(20.0);

                // Title
                ui.label(RichText::new("Benchmark Results").size(28.0).strong().color(Theme::ACCENT));
                ui.label(
                    RichText::new(format!(
                        "{} - {}",
                        run.machine_name,
                        run.timestamp.format("%Y-%m-%d %H:%M")
                    ))
                    .color(Theme::TEXT_SECONDARY),
                );

                ui.add_space(30.0);

                // Overall score
                ui.add(LargeScoreCard::new(
                    "Overall Score",
                    run.scores.overall,
                    run.scores.overall_max,
                    run.scores.rating,
                ));

                ui.add_space(30.0);

                // Category scores
                ui.label(RichText::new("Category Scores").size(20.0).strong());
                ui.add_space(16.0);

                ui.horizontal_wrapped(|ui| {
                    ui.add(ScoreCard::new(
                        "Project Operations",
                        run.scores.categories.project_operations.score,
                        run.scores.categories.project_operations.max_score,
                        run.scores.categories.project_operations.rating,
                    ));

                    ui.add(ScoreCard::new(
                        "Build Performance",
                        run.scores.categories.build_performance.score,
                        run.scores.categories.build_performance.max_score,
                        run.scores.categories.build_performance.rating,
                    ));

                    ui.add(ScoreCard::new(
                        "Responsiveness",
                        run.scores.categories.responsiveness.score,
                        run.scores.categories.responsiveness.max_score,
                        run.scores.categories.responsiveness.rating,
                    ));

                    if let Some(graphics) = &run.scores.categories.graphics {
                        ui.add(ScoreCard::new(
                            "Graphics",
                            graphics.score,
                            graphics.max_score,
                            graphics.rating,
                        ));
                    }
                });

                ui.add_space(30.0);

                // Detailed results
                ui.label(RichText::new("Detailed Results").size(20.0).strong());
                ui.add_space(16.0);

                Self::show_category_results(ui, "Project Operations", &run.results.project_operations);
                Self::show_category_results(ui, "Build Performance", &run.results.build_performance);
                Self::show_category_results(ui, "Responsiveness", &run.results.responsiveness);

                if let Some(graphics) = &run.results.graphics {
                    Self::show_category_results(ui, "Graphics", graphics);
                }

                ui.add_space(30.0);

                // Action buttons
                ui.horizontal(|ui| {
                    if ui
                        .button(RichText::new("Back to Home").size(14.0))
                        .clicked()
                    {
                        back_clicked = true;
                    }

                    ui.add_space(16.0);

                    if ui
                        .button(RichText::new("Export Results").size(14.0))
                        .clicked()
                    {
                        export_clicked = true;
                    }
                });

                ui.add_space(30.0);
            });
        });

        (back_clicked, export_clicked)
    }

    fn show_category_results(ui: &mut Ui, category_name: &str, results: &[crate::models::TestResult]) {
        if results.is_empty() {
            return;
        }

        egui::Frame::none()
            .fill(Theme::BG_CARD)
            .stroke(egui::Stroke::new(1.0, Theme::BORDER))
            .rounding(8.0)
            .inner_margin(16.0)
            .show(ui, |ui| {
                ui.set_min_width(600.0);

                ui.label(RichText::new(category_name).size(16.0).strong());
                ui.add_space(12.0);

                egui::Grid::new(format!("results_grid_{}", category_name))
                    .num_columns(4)
                    .spacing([20.0, 8.0])
                    .striped(true)
                    .show(ui, |ui| {
                        // Header
                        ui.label(RichText::new("Test").strong());
                        ui.label(RichText::new("Value").strong());
                        ui.label(RichText::new("Score").strong());
                        ui.label(RichText::new("").strong());
                        ui.end_row();

                        for result in results {
                            ui.label(&result.name);
                            ui.label(format!("{:.2} {}", result.value, result.unit));
                            ui.label(format!("{} / {}", result.score, result.max_score));

                            let percentage = result.score_percentage();
                            let rating = crate::models::Rating::from_percentage(percentage);
                            ui.label(
                                RichText::new(format!("{:.0}%", percentage))
                                    .color(Theme::rating_color(&rating)),
                            );
                            ui.end_row();
                        }
                    });
            });

        ui.add_space(16.0);
    }
}
