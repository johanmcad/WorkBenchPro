use egui::{Align, Layout, RichText, Ui};

use crate::models::{BenchmarkRun, Rating};
use crate::ui::widgets::ProgressBar;
use crate::ui::Theme;

/// Actions that can be triggered from the history view
pub enum HistoryAction {
    None,
    Back,
    ViewRun(usize),
    CompareRuns(usize, usize),
    DeleteRun(usize),
}

pub struct HistoryView;

impl HistoryView {
    pub fn show(
        ui: &mut Ui,
        runs: &[BenchmarkRun],
        selected_runs: &mut [bool],
    ) -> HistoryAction {
        let mut action = HistoryAction::None;

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.add_space(24.0);

                // Title
                ui.label(
                    RichText::new("Benchmark History")
                        .size(Theme::SIZE_TITLE)
                        .strong()
                        .color(Theme::ACCENT),
                );
                ui.add_space(4.0);
                ui.label(
                    RichText::new(format!("{} saved runs", runs.len()))
                        .size(Theme::SIZE_BODY)
                        .color(Theme::TEXT_SECONDARY),
                );

                ui.add_space(24.0);

                // Compare button (if 2 runs selected)
                let selected_count: usize = selected_runs.iter().filter(|&&s| s).count();
                if selected_count == 2 {
                    let indices: Vec<usize> = selected_runs
                        .iter()
                        .enumerate()
                        .filter(|(_, &s)| s)
                        .map(|(i, _)| i)
                        .collect();

                    let compare_btn = egui::Button::new(
                        RichText::new("Compare Selected Runs")
                            .size(Theme::SIZE_CARD)
                            .strong()
                            .color(egui::Color32::WHITE),
                    )
                    .min_size(egui::vec2(200.0, 44.0))
                    .fill(Theme::ACCENT)
                    .rounding(Theme::CARD_ROUNDING);

                    if ui.add(compare_btn).clicked() {
                        action = HistoryAction::CompareRuns(indices[0], indices[1]);
                    }
                    ui.add_space(16.0);
                } else if selected_count > 0 && selected_count < 2 {
                    ui.label(
                        RichText::new(format!("Select {} more to compare", 2 - selected_count))
                            .size(Theme::SIZE_BODY)
                            .color(Theme::TEXT_SECONDARY)
                            .italics(),
                    );
                    ui.add_space(16.0);
                }

                // History list
                if runs.is_empty() {
                    ui.add_space(48.0);

                    egui::Frame::none()
                        .fill(Theme::BG_CARD)
                        .stroke(egui::Stroke::new(1.0, Theme::BORDER))
                        .rounding(Theme::CARD_ROUNDING)
                        .inner_margin(32.0)
                        .show(ui, |ui| {
                            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                                ui.label(
                                    RichText::new("No benchmark history yet")
                                        .size(Theme::SIZE_SECTION)
                                        .color(Theme::TEXT_SECONDARY),
                                );
                                ui.add_space(8.0);
                                ui.label(
                                    RichText::new("Run a benchmark to see results here")
                                        .size(Theme::SIZE_BODY)
                                        .color(Theme::TEXT_SECONDARY),
                                );
                            });
                        });
                } else {
                    for (idx, run) in runs.iter().enumerate() {
                        let is_selected = selected_runs.get(idx).copied().unwrap_or(false);

                        let frame = if is_selected {
                            egui::Frame::none()
                                .fill(Theme::ACCENT.linear_multiply(0.1))
                                .stroke(egui::Stroke::new(2.0, Theme::ACCENT))
                                .rounding(Theme::CARD_ROUNDING)
                                .inner_margin(16.0)
                        } else {
                            egui::Frame::none()
                                .fill(Theme::BG_CARD)
                                .stroke(egui::Stroke::new(1.0, Theme::BORDER))
                                .rounding(Theme::CARD_ROUNDING)
                                .inner_margin(16.0)
                        };

                        frame.show(ui, |ui| {
                            ui.set_min_width(650.0);

                            ui.horizontal(|ui| {
                                // Checkbox for selection
                                let mut selected = is_selected;
                                if ui.checkbox(&mut selected, "").changed() {
                                    if let Some(s) = selected_runs.get_mut(idx) {
                                        *s = selected;
                                    }
                                }

                                ui.add_space(12.0);

                                // Run info
                                ui.vertical(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            RichText::new(&run.machine_name)
                                                .size(Theme::SIZE_CARD)
                                                .strong()
                                                .color(Theme::TEXT_PRIMARY),
                                        );
                                        ui.add_space(12.0);
                                        ui.label(
                                            RichText::new(
                                                run.timestamp.format("%Y-%m-%d %H:%M").to_string(),
                                            )
                                            .size(Theme::SIZE_BODY)
                                            .color(Theme::TEXT_SECONDARY),
                                        );
                                    });

                                    ui.add_space(8.0);

                                    // Score with progress bar
                                    let percentage = (run.scores.overall as f64
                                        / run.scores.overall_max as f64)
                                        * 100.0;
                                    let rating = Rating::from_percentage(percentage);

                                    ui.horizontal(|ui| {
                                        ui.label(
                                            RichText::new(format!("{}", run.scores.overall))
                                                .size(Theme::SIZE_SECTION)
                                                .strong()
                                                .color(Theme::rating_color(&rating)),
                                        );
                                        ui.label(
                                            RichText::new(format!("/ {}", run.scores.overall_max))
                                                .size(Theme::SIZE_BODY)
                                                .color(Theme::TEXT_SECONDARY),
                                        );
                                        ui.add_space(16.0);

                                        // Rating badge
                                        egui::Frame::none()
                                            .fill(Theme::rating_bg_color(&run.scores.rating))
                                            .rounding(Theme::BADGE_ROUNDING)
                                            .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                                            .show(ui, |ui| {
                                                ui.label(
                                                    RichText::new(run.scores.rating.label())
                                                        .size(Theme::SIZE_CAPTION)
                                                        .color(Theme::rating_color(&run.scores.rating)),
                                                );
                                            });
                                    });

                                    ui.add_space(8.0);

                                    // Progress bar
                                    ui.add(
                                        ProgressBar::new(percentage as f32 / 100.0)
                                            .rating(rating)
                                            .width(300.0)
                                            .small(),
                                    );
                                });

                                // Action buttons (right side)
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    // Delete button
                                    let delete_btn = egui::Button::new(
                                        RichText::new("Delete")
                                            .size(Theme::SIZE_CAPTION)
                                            .color(Theme::ERROR),
                                    )
                                    .rounding(Theme::BADGE_ROUNDING);

                                    if ui.add(delete_btn).clicked() {
                                        action = HistoryAction::DeleteRun(idx);
                                    }

                                    ui.add_space(8.0);

                                    // View button
                                    let view_btn = egui::Button::new(
                                        RichText::new("View").size(Theme::SIZE_CAPTION),
                                    )
                                    .rounding(Theme::BADGE_ROUNDING);

                                    if ui.add(view_btn).clicked() {
                                        action = HistoryAction::ViewRun(idx);
                                    }
                                });
                            });
                        });

                        ui.add_space(8.0);
                    }
                }

                ui.add_space(32.0);

                // Back button
                let back_btn = egui::Button::new(
                    RichText::new("Back to Home").size(Theme::SIZE_BODY),
                )
                .min_size(egui::vec2(140.0, 40.0))
                .rounding(Theme::CARD_ROUNDING);

                if ui.add(back_btn).clicked() {
                    action = HistoryAction::Back;
                }

                ui.add_space(32.0);
            });
        });

        action
    }
}
