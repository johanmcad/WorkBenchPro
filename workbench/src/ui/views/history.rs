use egui::{Align, Layout, RichText, Ui};

use crate::models::{BenchmarkRun, Rating};
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
                ui.add_space(20.0);

                // Title
                ui.label(RichText::new("Benchmark History").size(28.0).strong().color(Theme::ACCENT));
                ui.label(
                    RichText::new(format!("{} saved runs", runs.len()))
                        .color(Theme::TEXT_SECONDARY),
                );

                ui.add_space(20.0);

                // Compare button (if 2 runs selected)
                let selected_count: usize = selected_runs.iter().filter(|&&s| s).count();
                if selected_count == 2 {
                    let indices: Vec<usize> = selected_runs
                        .iter()
                        .enumerate()
                        .filter(|(_, &s)| s)
                        .map(|(i, _)| i)
                        .collect();

                    if ui
                        .button(RichText::new("Compare Selected Runs").size(16.0))
                        .clicked()
                    {
                        action = HistoryAction::CompareRuns(indices[0], indices[1]);
                    }
                    ui.add_space(10.0);
                } else if selected_count > 0 {
                    ui.label(
                        RichText::new(format!("Select {} more to compare", 2 - selected_count))
                            .color(Theme::TEXT_SECONDARY)
                            .italics(),
                    );
                    ui.add_space(10.0);
                }

                ui.add_space(10.0);

                // History list
                if runs.is_empty() {
                    ui.add_space(50.0);
                    ui.label(
                        RichText::new("No benchmark history yet")
                            .size(16.0)
                            .color(Theme::TEXT_SECONDARY),
                    );
                    ui.label(
                        RichText::new("Run a benchmark to see results here")
                            .color(Theme::TEXT_SECONDARY),
                    );
                } else {
                    for (idx, run) in runs.iter().enumerate() {
                        let is_selected = selected_runs.get(idx).copied().unwrap_or(false);

                        let frame = if is_selected {
                            egui::Frame::none()
                                .fill(Theme::ACCENT.linear_multiply(0.2))
                                .stroke(egui::Stroke::new(2.0, Theme::ACCENT))
                                .rounding(8.0)
                                .inner_margin(16.0)
                        } else {
                            egui::Frame::none()
                                .fill(Theme::BG_CARD)
                                .stroke(egui::Stroke::new(1.0, Theme::BORDER))
                                .rounding(8.0)
                                .inner_margin(16.0)
                        };

                        frame.show(ui, |ui| {
                            ui.set_min_width(600.0);

                            ui.horizontal(|ui| {
                                // Checkbox for selection
                                let mut selected = is_selected;
                                if ui.checkbox(&mut selected, "").changed() {
                                    if let Some(s) = selected_runs.get_mut(idx) {
                                        *s = selected;
                                    }
                                }

                                ui.add_space(10.0);

                                // Run info
                                ui.vertical(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            RichText::new(&run.machine_name)
                                                .size(16.0)
                                                .strong(),
                                        );
                                        ui.label(
                                            RichText::new(
                                                run.timestamp.format("%Y-%m-%d %H:%M").to_string(),
                                            )
                                            .color(Theme::TEXT_SECONDARY),
                                        );
                                    });

                                    ui.add_space(4.0);

                                    ui.horizontal(|ui| {
                                        // Overall score
                                        let percentage = (run.scores.overall as f64
                                            / run.scores.overall_max as f64)
                                            * 100.0;
                                        let rating = Rating::from_percentage(percentage);

                                        ui.label(
                                            RichText::new(format!(
                                                "Score: {} / {} ({:.0}%)",
                                                run.scores.overall,
                                                run.scores.overall_max,
                                                percentage
                                            ))
                                            .color(Theme::rating_color(&rating)),
                                        );

                                        ui.add_space(20.0);

                                        ui.label(
                                            RichText::new(format!("{:?}", run.scores.rating))
                                                .color(Theme::rating_color(&run.scores.rating)),
                                        );
                                    });
                                });

                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    // Delete button
                                    if ui
                                        .small_button(RichText::new("Delete").color(Theme::ERROR))
                                        .clicked()
                                    {
                                        action = HistoryAction::DeleteRun(idx);
                                    }

                                    ui.add_space(10.0);

                                    // View button
                                    if ui.small_button("View").clicked() {
                                        action = HistoryAction::ViewRun(idx);
                                    }
                                });
                            });
                        });

                        ui.add_space(8.0);
                    }
                }

                ui.add_space(30.0);

                // Back button
                if ui
                    .button(RichText::new("Back to Home").size(14.0))
                    .clicked()
                {
                    action = HistoryAction::Back;
                }

                ui.add_space(30.0);
            });
        });

        action
    }
}
