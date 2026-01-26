//! Community Browser View - Browse and compare against community benchmark results

use egui::{Align, Layout, RichText, Ui};

use crate::cloud::CommunityRun;
use crate::ui::Theme;

/// Actions that can be triggered from the community browser
pub enum CommunityAction {
    None,
    Back,
    SelectForComparison(String), // Remote run ID
    Refresh,
    FilterChanged,
}

/// State for filter controls
#[derive(Default, Clone)]
pub struct CommunityFilters {
    pub cpu_filter: String,
    pub os_filter: String,
    pub min_memory: String,
}

pub struct CommunityView;

impl CommunityView {
    pub fn show(
        ui: &mut Ui,
        runs: &[CommunityRun],
        filters: &mut CommunityFilters,
        loading: bool,
        error: Option<&str>,
    ) -> CommunityAction {
        let mut action = CommunityAction::None;

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.add_space(12.0);

                // Title
                ui.label(
                    RichText::new("Community Benchmarks")
                        .size(Theme::SIZE_SECTION)
                        .strong()
                        .color(Theme::ACCENT),
                );
                ui.label(
                    RichText::new("Compare your results against the community")
                        .size(Theme::SIZE_CAPTION)
                        .color(Theme::TEXT_SECONDARY),
                );

                ui.add_space(12.0);

                // Filter controls
                egui::Frame::none()
                    .fill(Theme::BG_CARD)
                    .stroke(egui::Stroke::new(1.0, Theme::BORDER))
                    .rounding(Theme::CARD_ROUNDING)
                    .inner_margin(12.0)
                    .show(ui, |ui| {
                        ui.set_min_width(600.0);

                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new("Filters:")
                                    .size(Theme::SIZE_CAPTION)
                                    .color(Theme::TEXT_SECONDARY),
                            );

                            ui.add_space(8.0);

                            // CPU filter
                            ui.label(RichText::new("CPU:").size(Theme::SIZE_CAPTION));
                            let cpu_response = ui.add(
                                egui::TextEdit::singleline(&mut filters.cpu_filter)
                                    .desired_width(120.0)
                                    .hint_text("e.g. Ryzen"),
                            );

                            ui.add_space(8.0);

                            // OS filter
                            ui.label(RichText::new("OS:").size(Theme::SIZE_CAPTION));
                            let os_response = ui.add(
                                egui::TextEdit::singleline(&mut filters.os_filter)
                                    .desired_width(100.0)
                                    .hint_text("e.g. Windows"),
                            );

                            ui.add_space(8.0);

                            // Memory filter
                            ui.label(RichText::new("Min RAM:").size(Theme::SIZE_CAPTION));
                            let mem_response = ui.add(
                                egui::TextEdit::singleline(&mut filters.min_memory)
                                    .desired_width(60.0)
                                    .hint_text("GB"),
                            );

                            ui.add_space(8.0);

                            // Apply button
                            let apply_btn = egui::Button::new(
                                RichText::new("Apply").size(Theme::SIZE_CAPTION),
                            )
                            .rounding(Theme::BADGE_ROUNDING);

                            if ui.add(apply_btn).clicked()
                                || cpu_response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))
                                || os_response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))
                                || mem_response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))
                            {
                                action = CommunityAction::FilterChanged;
                            }

                            // Refresh button
                            let refresh_btn = egui::Button::new(
                                RichText::new("Refresh").size(Theme::SIZE_CAPTION),
                            )
                            .rounding(Theme::BADGE_ROUNDING);

                            if ui.add(refresh_btn).clicked() {
                                action = CommunityAction::Refresh;
                            }
                        });
                    });

                ui.add_space(12.0);

                // Loading indicator
                if loading {
                    ui.add_space(16.0);
                    ui.spinner();
                    ui.label(
                        RichText::new("Loading community results...")
                            .size(Theme::SIZE_BODY)
                            .color(Theme::TEXT_SECONDARY),
                    );
                    ui.add_space(16.0);
                } else if let Some(err) = error {
                    // Error message
                    ui.add_space(8.0);
                    egui::Frame::none()
                        .fill(Theme::ERROR.linear_multiply(0.1))
                        .stroke(egui::Stroke::new(1.0, Theme::ERROR))
                        .rounding(Theme::CARD_ROUNDING)
                        .inner_margin(12.0)
                        .show(ui, |ui| {
                            ui.label(
                                RichText::new(format!("Error: {}", err))
                                    .size(Theme::SIZE_BODY)
                                    .color(Theme::ERROR),
                            );
                        });
                    ui.add_space(8.0);
                } else if runs.is_empty() {
                    // Empty state
                    ui.add_space(16.0);
                    egui::Frame::none()
                        .fill(Theme::BG_CARD)
                        .stroke(egui::Stroke::new(1.0, Theme::BORDER))
                        .rounding(Theme::CARD_ROUNDING)
                        .inner_margin(16.0)
                        .show(ui, |ui| {
                            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                                ui.label(
                                    RichText::new("No community results yet")
                                        .size(Theme::SIZE_CARD)
                                        .color(Theme::TEXT_SECONDARY),
                                );
                                ui.label(
                                    RichText::new("Be the first to upload your benchmark!")
                                        .size(Theme::SIZE_CAPTION)
                                        .color(Theme::TEXT_SECONDARY),
                                );
                            });
                        });
                    ui.add_space(16.0);
                } else {
                    // Results count
                    ui.label(
                        RichText::new(format!("{} results", runs.len()))
                            .size(Theme::SIZE_CAPTION)
                            .color(Theme::TEXT_SECONDARY),
                    );
                    ui.add_space(4.0);

                    // Results list
                    for run in runs {
                        let frame = egui::Frame::none()
                            .fill(Theme::BG_CARD)
                            .stroke(egui::Stroke::new(1.0, Theme::BORDER))
                            .rounding(Theme::CARD_ROUNDING)
                            .inner_margin(8.0);

                        frame.show(ui, |ui| {
                            ui.set_min_width(600.0);

                            ui.horizontal(|ui| {
                                // Main info
                                ui.vertical(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            RichText::new(&run.display_name)
                                                .size(Theme::SIZE_BODY)
                                                .strong()
                                                .color(Theme::TEXT_PRIMARY),
                                        );
                                        ui.add_space(8.0);
                                        ui.label(
                                            RichText::new(
                                                run.uploaded_at.format("%Y-%m-%d").to_string(),
                                            )
                                            .size(Theme::SIZE_CAPTION)
                                            .color(Theme::TEXT_SECONDARY),
                                        );
                                    });

                                    // System specs
                                    ui.horizontal(|ui| {
                                        // CPU badge
                                        egui::Frame::none()
                                            .fill(Theme::BG_SECONDARY)
                                            .rounding(Theme::BADGE_ROUNDING)
                                            .inner_margin(egui::Margin::symmetric(6.0, 2.0))
                                            .show(ui, |ui| {
                                                ui.label(
                                                    RichText::new(&run.cpu_name)
                                                        .size(Theme::SIZE_CAPTION)
                                                        .color(Theme::ACCENT),
                                                );
                                            });

                                        ui.add_space(4.0);

                                        // Cores/threads
                                        ui.label(
                                            RichText::new(format!(
                                                "{}C/{}T",
                                                run.cpu_cores, run.cpu_threads
                                            ))
                                            .size(Theme::SIZE_CAPTION)
                                            .color(Theme::TEXT_SECONDARY),
                                        );

                                        ui.add_space(4.0);

                                        // RAM
                                        ui.label(
                                            RichText::new(format!("{:.0} GB", run.memory_gb))
                                                .size(Theme::SIZE_CAPTION)
                                                .color(Theme::TEXT_SECONDARY),
                                        );

                                        ui.add_space(4.0);

                                        // OS
                                        ui.label(
                                            RichText::new(&run.os_name)
                                                .size(Theme::SIZE_CAPTION)
                                                .color(Theme::TEXT_SECONDARY),
                                        );

                                        if let Some(ref storage) = run.storage_type {
                                            ui.add_space(4.0);
                                            ui.label(
                                                RichText::new(storage)
                                                    .size(Theme::SIZE_CAPTION)
                                                    .color(Theme::TEXT_SECONDARY),
                                            );
                                        }
                                    });
                                });

                                // Compare button (right side)
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    let compare_btn = egui::Button::new(
                                        RichText::new("Compare")
                                            .size(Theme::SIZE_CAPTION)
                                            .color(egui::Color32::WHITE),
                                    )
                                    .fill(Theme::ACCENT)
                                    .rounding(Theme::BADGE_ROUNDING);

                                    if ui.add(compare_btn).clicked() {
                                        action = CommunityAction::SelectForComparison(run.id.clone());
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
                    RichText::new("Back").size(Theme::SIZE_BODY),
                )
                .min_size(egui::vec2(100.0, 32.0))
                .rounding(Theme::CARD_ROUNDING);

                if ui.add(back_btn).clicked() {
                    action = CommunityAction::Back;
                }

                ui.add_space(12.0);
            });
        });

        action
    }
}
