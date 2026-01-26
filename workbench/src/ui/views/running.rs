use egui::{Align, Layout, RichText, Ui};

use crate::ui::widgets::ProgressBar;
use crate::ui::Theme;

/// Running View matching 05-ui-design.md spec:
/// - Overall progress bar
/// - Category list with status
/// - Current test name + progress
/// - Cancel button
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

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.add_space(16.0);

                // Title
                ui.label(
                    RichText::new("Running Benchmarks...")
                        .size(Theme::SIZE_SECTION)
                        .strong()
                        .color(Theme::TEXT_PRIMARY),
                );

                ui.add_space(16.0);

                // Overall Progress Card
                egui::Frame::none()
                    .fill(Theme::BG_CARD)
                    .stroke(egui::Stroke::new(1.0, Theme::BORDER))
                    .rounding(Theme::CARD_ROUNDING)
                    .inner_margin(12.0)
                    .show(ui, |ui| {
                        ui.set_min_width(420.0);

                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new("Overall Progress")
                                    .size(Theme::SIZE_BODY)
                                    .color(Theme::TEXT_SECONDARY),
                            );
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                ui.label(
                                    RichText::new(format!("{:.0}%", overall_progress * 100.0))
                                        .size(Theme::SIZE_CARD)
                                        .strong()
                                        .color(Theme::ACCENT),
                                );
                            });
                        });

                        ui.add_space(6.0);

                        // Large progress bar
                        ui.add(ProgressBar::new(overall_progress).height(12.0).width(420.0));
                    });

                ui.add_space(8.0);

                // Current Test Card
                egui::Frame::none()
                    .fill(Theme::BG_CARD)
                    .stroke(egui::Stroke::new(1.0, Theme::BORDER))
                    .rounding(Theme::CARD_ROUNDING)
                    .inner_margin(12.0)
                    .show(ui, |ui| {
                        ui.set_min_width(420.0);

                        ui.label(
                            RichText::new("Current Test")
                                .size(Theme::SIZE_BODY)
                                .color(Theme::TEXT_SECONDARY),
                        );

                        ui.add_space(4.0);

                        let test_name = if current_test.is_empty() {
                            "Initializing..."
                        } else {
                            current_test
                        };

                        ui.label(
                            RichText::new(test_name)
                                .size(Theme::SIZE_CARD)
                                .strong()
                                .color(Theme::TEXT_PRIMARY),
                        );

                        ui.label(
                            RichText::new(current_message)
                                .size(Theme::SIZE_CAPTION)
                                .color(Theme::TEXT_SECONDARY)
                                .italics(),
                        );
                    });

                ui.add_space(8.0);

                // Completed Tests Card (scrollable)
                if !completed_tests.is_empty() {
                    egui::Frame::none()
                        .fill(Theme::BG_CARD)
                        .stroke(egui::Stroke::new(1.0, Theme::BORDER))
                        .rounding(Theme::CARD_ROUNDING)
                        .inner_margin(12.0)
                        .show(ui, |ui| {
                            ui.set_min_width(420.0);

                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new("Completed Tests")
                                        .size(Theme::SIZE_BODY)
                                        .color(Theme::TEXT_SECONDARY),
                                );
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    ui.label(
                                        RichText::new(format!("{}", completed_tests.len()))
                                            .size(Theme::SIZE_BODY)
                                            .strong()
                                            .color(Theme::SUCCESS),
                                    );
                                });
                            });

                            ui.add_space(4.0);

                            egui::ScrollArea::vertical()
                                .max_height(300.0)
                                .show(ui, |ui| {
                                    for test in completed_tests.iter().rev() {
                                        ui.horizontal(|ui| {
                                            ui.label(
                                                RichText::new("✓")
                                                    .size(Theme::SIZE_CAPTION)
                                                    .color(Theme::SUCCESS),
                                            );
                                            ui.add_space(4.0);
                                            ui.label(
                                                RichText::new(test)
                                                    .size(Theme::SIZE_CAPTION)
                                                    .color(Theme::TEXT_PRIMARY),
                                            );
                                        });
                                    }
                                });
                        });
                }

                ui.add_space(12.0);

                // Cancel Button
                let cancel_button = egui::Button::new(
                    RichText::new("Cancel")
                        .size(Theme::SIZE_BODY)
                        .color(Theme::ERROR),
                )
                .min_size(egui::vec2(100.0, 32.0))
                .rounding(Theme::CARD_ROUNDING);

                if ui.add(cancel_button).clicked() {
                    cancel_clicked = true;
                }

                ui.add_space(12.0);
            });
        });

        // Request continuous repaints while running
        ui.ctx().request_repaint();

        cancel_clicked
    }
}
