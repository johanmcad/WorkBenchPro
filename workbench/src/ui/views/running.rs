use egui::{Align, Layout, RichText, Ui};

use crate::ui::widgets::ProgressBar;
use crate::ui::Theme;

pub struct RunningView;

impl RunningView {
    /// Returns true if cancel button was clicked
    pub fn show(
        ui: &mut Ui,
        overall_progress: f32,
        current_test: &str,
        current_message: &str,
        completed_tests: &[String],
    ) -> bool {
        let mut cancel_clicked = false;

        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            ui.add_space(40.0);

            // Title
            ui.label(RichText::new("Running Benchmarks...").size(24.0).strong());

            ui.add_space(30.0);

            // Overall progress
            egui::Frame::none()
                .fill(Theme::BG_CARD)
                .stroke(egui::Stroke::new(1.0, Theme::BORDER))
                .rounding(8.0)
                .inner_margin(20.0)
                .show(ui, |ui| {
                    ui.set_min_width(500.0);

                    ui.label(RichText::new("Overall Progress").size(16.0).color(Theme::TEXT_SECONDARY));
                    ui.add_space(8.0);

                    ui.add(ProgressBar::new(overall_progress).height(12.0));

                    ui.add_space(4.0);
                    ui.label(
                        RichText::new(format!("{:.0}%", overall_progress * 100.0))
                            .color(Theme::TEXT_SECONDARY),
                    );
                });

            ui.add_space(20.0);

            // Current test
            egui::Frame::none()
                .fill(Theme::BG_CARD)
                .stroke(egui::Stroke::new(1.0, Theme::BORDER))
                .rounding(8.0)
                .inner_margin(20.0)
                .show(ui, |ui| {
                    ui.set_min_width(500.0);

                    ui.label(RichText::new("Current Test").size(16.0).color(Theme::TEXT_SECONDARY));
                    ui.add_space(8.0);

                    let test_name = if current_test.is_empty() {
                        "Initializing..."
                    } else {
                        current_test
                    };
                    ui.label(RichText::new(test_name).size(18.0).strong());

                    ui.add_space(4.0);
                    ui.label(RichText::new(current_message).color(Theme::TEXT_SECONDARY));
                });

            ui.add_space(20.0);

            // Completed tests
            if !completed_tests.is_empty() {
                egui::Frame::none()
                    .fill(Theme::BG_CARD)
                    .stroke(egui::Stroke::new(1.0, Theme::BORDER))
                    .rounding(8.0)
                    .inner_margin(20.0)
                    .show(ui, |ui| {
                        ui.set_min_width(500.0);

                        ui.label(
                            RichText::new("Completed Tests")
                                .size(16.0)
                                .color(Theme::TEXT_SECONDARY),
                        );
                        ui.add_space(8.0);

                        for test in completed_tests {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("✓").color(Theme::SCORE_EXCELLENT));
                                ui.label(test);
                            });
                        }
                    });
            }

            ui.add_space(30.0);

            // Cancel button
            let button = egui::Button::new(RichText::new("Cancel").size(14.0))
                .min_size(egui::vec2(120.0, 36.0));

            if ui.add(button).clicked() {
                cancel_clicked = true;
            }
        });

        // Request continuous repaints while running
        ui.ctx().request_repaint();

        cancel_clicked
    }
}
