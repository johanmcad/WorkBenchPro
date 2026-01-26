use egui::{Align, Layout, RichText, Ui};

use crate::models::BenchmarkRun;
use crate::ui::Theme;

/// Actions that can be triggered from the history view
pub enum HistoryAction {
    None,
    Back,
    ViewRun(usize),
    CompareRuns(usize, usize),
    CompareOnline(usize),       // Compare run at index against community
    CommunityComparison(usize), // View community comparison for run at index
    Upload(usize),              // Upload run at index to community
    RemoveUpload(usize),        // Remove uploaded run from community
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
                ui.add_space(12.0);

                // Title
                ui.label(
                    RichText::new("Benchmark History")
                        .size(Theme::SIZE_SECTION)
                        .strong()
                        .color(Theme::ACCENT),
                );
                ui.label(
                    RichText::new(format!("{} saved runs", runs.len()))
                        .size(Theme::SIZE_CAPTION)
                        .color(Theme::TEXT_SECONDARY),
                );

                ui.add_space(8.0);

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
                            .size(Theme::SIZE_BODY)
                            .strong()
                            .color(egui::Color32::WHITE),
                    )
                    .min_size(egui::vec2(160.0, 32.0))
                    .fill(Theme::ACCENT)
                    .rounding(Theme::CARD_ROUNDING);

                    if ui.add(compare_btn).clicked() {
                        action = HistoryAction::CompareRuns(indices[0], indices[1]);
                    }
                    ui.add_space(8.0);
                } else if selected_count > 0 && selected_count < 2 {
                    ui.label(
                        RichText::new(format!("Select {} more to compare", 2 - selected_count))
                            .size(Theme::SIZE_CAPTION)
                            .color(Theme::TEXT_SECONDARY)
                            .italics(),
                    );
                    ui.add_space(8.0);
                }

                // History list
                if runs.is_empty() {
                    ui.add_space(16.0);

                    egui::Frame::none()
                        .fill(Theme::BG_CARD)
                        .stroke(egui::Stroke::new(1.0, Theme::BORDER))
                        .rounding(Theme::CARD_ROUNDING)
                        .inner_margin(16.0)
                        .show(ui, |ui| {
                            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                                ui.label(
                                    RichText::new("No benchmark history yet")
                                        .size(Theme::SIZE_CARD)
                                        .color(Theme::TEXT_SECONDARY),
                                );
                                ui.label(
                                    RichText::new("Run a benchmark to see results here")
                                        .size(Theme::SIZE_CAPTION)
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
                                .inner_margin(8.0)
                        } else {
                            egui::Frame::none()
                                .fill(Theme::BG_CARD)
                                .stroke(egui::Stroke::new(1.0, Theme::BORDER))
                                .rounding(Theme::CARD_ROUNDING)
                                .inner_margin(8.0)
                        };

                        frame.show(ui, |ui| {
                            ui.set_min_width(550.0);

                            ui.horizontal(|ui| {
                                // Checkbox for selection
                                let mut selected = is_selected;
                                if ui.checkbox(&mut selected, "").changed() {
                                    if let Some(s) = selected_runs.get_mut(idx) {
                                        *s = selected;
                                    }
                                }

                                ui.add_space(4.0);

                                // Run info
                                ui.vertical(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            RichText::new(&run.machine_name)
                                                .size(Theme::SIZE_BODY)
                                                .strong()
                                                .color(Theme::TEXT_PRIMARY),
                                        );
                                        ui.add_space(8.0);
                                        ui.label(
                                            RichText::new(
                                                run.timestamp.format("%Y-%m-%d %H:%M").to_string(),
                                            )
                                            .size(Theme::SIZE_CAPTION)
                                            .color(Theme::TEXT_SECONDARY),
                                        );
                                    });

                                    // Test count summary
                                    let total_tests = run.results.project_operations.len()
                                        + run.results.build_performance.len()
                                        + run.results.responsiveness.len();

                                    ui.horizontal(|ui| {
                                        // Tests completed badge
                                        egui::Frame::none()
                                            .fill(Theme::BG_SECONDARY)
                                            .rounding(Theme::BADGE_ROUNDING)
                                            .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                                            .show(ui, |ui| {
                                                ui.label(
                                                    RichText::new(format!("{} tests", total_tests))
                                                        .size(Theme::SIZE_BODY)
                                                        .strong()
                                                        .color(Theme::ACCENT),
                                                );
                                            });

                                        ui.add_space(8.0);

                                        // Category breakdown
                                        if !run.results.project_operations.is_empty() {
                                            ui.label(
                                                RichText::new(format!(
                                                    "Proj: {}",
                                                    run.results.project_operations.len()
                                                ))
                                                .size(Theme::SIZE_CAPTION)
                                                .color(Theme::TEXT_SECONDARY),
                                            );
                                        }
                                        if !run.results.build_performance.is_empty() {
                                            ui.label(
                                                RichText::new(format!(
                                                    "Build: {}",
                                                    run.results.build_performance.len()
                                                ))
                                                .size(Theme::SIZE_CAPTION)
                                                .color(Theme::TEXT_SECONDARY),
                                            );
                                        }
                                        if !run.results.responsiveness.is_empty() {
                                            ui.label(
                                                RichText::new(format!(
                                                    "Resp: {}",
                                                    run.results.responsiveness.len()
                                                ))
                                                .size(Theme::SIZE_CAPTION)
                                                .color(Theme::TEXT_SECONDARY),
                                            );
                                        }
                                    });
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

                                    ui.add_space(4.0);

                                    // Upload button (only if not already uploaded)
                                    if run.uploaded_at.is_none() {
                                        let upload_btn = egui::Button::new(
                                            RichText::new("Upload")
                                                .size(Theme::SIZE_CAPTION)
                                                .color(egui::Color32::WHITE),
                                        )
                                        .fill(Theme::SUCCESS)
                                        .rounding(Theme::BADGE_ROUNDING);

                                        if ui.add(upload_btn).clicked() {
                                            action = HistoryAction::Upload(idx);
                                        }

                                        ui.add_space(4.0);
                                    } else {
                                        // Show remove upload button
                                        let remove_btn = egui::Button::new(
                                            RichText::new("Remove Upload")
                                                .size(Theme::SIZE_CAPTION)
                                                .color(Theme::WARNING),
                                        )
                                        .rounding(Theme::BADGE_ROUNDING);

                                        if ui.add(remove_btn).clicked() {
                                            action = HistoryAction::RemoveUpload(idx);
                                        }

                                        ui.add_space(4.0);
                                    }

                                    // Community Stats button (only if uploaded)
                                    if run.uploaded_at.is_some() {
                                        let stats_btn = egui::Button::new(
                                            RichText::new("Community Stats")
                                                .size(Theme::SIZE_CAPTION)
                                                .color(egui::Color32::WHITE),
                                        )
                                        .fill(Theme::SUCCESS)
                                        .rounding(Theme::BADGE_ROUNDING);

                                        if ui.add(stats_btn).clicked() {
                                            action = HistoryAction::CommunityComparison(idx);
                                        }

                                        ui.add_space(4.0);
                                    }

                                    // Compare Online button
                                    let online_btn = egui::Button::new(
                                        RichText::new("Compare Online")
                                            .size(Theme::SIZE_CAPTION)
                                            .color(egui::Color32::WHITE),
                                    )
                                    .fill(Theme::ACCENT)
                                    .rounding(Theme::BADGE_ROUNDING);

                                    if ui.add(online_btn).clicked() {
                                        action = HistoryAction::CompareOnline(idx);
                                    }

                                    ui.add_space(4.0);

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

                        ui.add_space(4.0);
                    }
                }

                ui.add_space(12.0);

                // Back button
                let back_btn = egui::Button::new(
                    RichText::new("Back to Home").size(Theme::SIZE_BODY),
                )
                .min_size(egui::vec2(100.0, 32.0))
                .rounding(Theme::CARD_ROUNDING);

                if ui.add(back_btn).clicked() {
                    action = HistoryAction::Back;
                }

                ui.add_space(12.0);
            });
        });

        action
    }
}
