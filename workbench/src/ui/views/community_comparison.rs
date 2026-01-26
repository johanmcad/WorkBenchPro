//! Community Comparison View - shows distribution plots for benchmark tests

use egui::{Align, Color32, Layout, RichText, Ui};
use egui_plot::{Bar, BarChart, Legend, Plot, VLine};

use crate::cloud::{PercentileRank, TestStatistics};
use crate::ui::Theme;

/// Actions from the community comparison view
pub enum CommunityComparisonAction {
    None,
    Back,
    Refresh,
}

/// Community Comparison View
pub struct CommunityComparisonView;

impl CommunityComparisonView {
    pub fn show(
        ui: &mut Ui,
        statistics: &[TestStatistics],
        percentile_ranks: &[PercentileRank],
        loading: bool,
        error: Option<&str>,
        run_name: &str,
    ) -> CommunityComparisonAction {
        let mut action = CommunityComparisonAction::None;

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.add_space(12.0);

                // Title
                ui.label(
                    RichText::new("Community Comparison")
                        .size(Theme::SIZE_SECTION)
                        .strong()
                        .color(Theme::ACCENT),
                );
                ui.label(
                    RichText::new(format!("Comparing: {}", run_name))
                        .size(Theme::SIZE_CAPTION)
                        .color(Theme::TEXT_SECONDARY),
                );

                ui.add_space(8.0);

                // Loading state
                if loading {
                    ui.add_space(16.0);
                    ui.spinner();
                    ui.label(
                        RichText::new("Loading statistics...")
                            .size(Theme::SIZE_BODY)
                            .color(Theme::TEXT_SECONDARY),
                    );
                    ui.add_space(16.0);
                } else if let Some(err) = error {
                    // Error state
                    ui.add_space(16.0);
                    ui.label(
                        RichText::new(format!("Error: {}", err))
                            .size(Theme::SIZE_BODY)
                            .color(Theme::ERROR),
                    );

                    ui.add_space(8.0);
                    let retry_btn = egui::Button::new(
                        RichText::new("Retry").size(Theme::SIZE_BODY),
                    )
                    .min_size(egui::vec2(80.0, 28.0))
                    .rounding(Theme::BADGE_ROUNDING);

                    if ui.add(retry_btn).clicked() {
                        action = CommunityComparisonAction::Refresh;
                    }
                    ui.add_space(16.0);
                } else if statistics.is_empty() {
                    // No data
                    ui.add_space(16.0);
                    egui::Frame::none()
                        .fill(Theme::BG_CARD)
                        .stroke(egui::Stroke::new(1.0, Theme::BORDER))
                        .rounding(Theme::CARD_ROUNDING)
                        .inner_margin(16.0)
                        .show(ui, |ui| {
                            ui.label(
                                RichText::new("No community data available")
                                    .size(Theme::SIZE_BODY)
                                    .color(Theme::TEXT_SECONDARY),
                            );
                            ui.label(
                                RichText::new("Upload more benchmarks to see comparisons")
                                    .size(Theme::SIZE_CAPTION)
                                    .color(Theme::TEXT_SECONDARY),
                            );
                        });
                    ui.add_space(16.0);
                } else {
                    // Summary card
                    Self::show_summary(ui, percentile_ranks);

                    ui.add_space(12.0);

                    // Rankings grid
                    Self::show_rankings_grid(ui, percentile_ranks);

                    ui.add_space(12.0);

                    // Distribution charts
                    ui.label(
                        RichText::new("Distribution Charts")
                            .size(Theme::SIZE_CARD)
                            .strong()
                            .color(Theme::TEXT_PRIMARY),
                    );
                    ui.add_space(8.0);

                    for stat in statistics {
                        let percentile = percentile_ranks
                            .iter()
                            .find(|p| p.test_id == stat.test_id);

                        Self::show_distribution_chart(ui, stat, percentile);
                        ui.add_space(8.0);
                    }
                }

                ui.add_space(12.0);

                // Back button
                let back_btn = egui::Button::new(
                    RichText::new("Back to History").size(Theme::SIZE_BODY),
                )
                .min_size(egui::vec2(100.0, 32.0))
                .rounding(Theme::CARD_ROUNDING);

                if ui.add(back_btn).clicked() {
                    action = CommunityComparisonAction::Back;
                }

                ui.add_space(12.0);
            });
        });

        action
    }

    fn show_summary(ui: &mut Ui, percentile_ranks: &[PercentileRank]) {
        if percentile_ranks.is_empty() {
            return;
        }

        let total = percentile_ranks.len();
        let avg_percentile: f64 =
            percentile_ranks.iter().map(|p| p.percentile_rank).sum::<f64>() / total as f64;
        let top_25_count = percentile_ranks
            .iter()
            .filter(|p| (100.0 - p.percentile_rank) <= 25.0)
            .count();

        egui::Frame::none()
            .fill(Theme::BG_CARD)
            .stroke(egui::Stroke::new(1.0, Theme::BORDER))
            .rounding(Theme::CARD_ROUNDING)
            .inner_margin(12.0)
            .show(ui, |ui| {
                ui.set_min_width(500.0);

                ui.horizontal(|ui| {
                    // Average percentile
                    egui::Frame::none()
                        .fill(Theme::BG_SECONDARY)
                        .rounding(Theme::BADGE_ROUNDING)
                        .inner_margin(egui::Margin::symmetric(16.0, 8.0))
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                ui.label(
                                    RichText::new(format!("{:.0}%", avg_percentile))
                                        .size(Theme::SIZE_SECTION)
                                        .strong()
                                        .color(Theme::ACCENT),
                                );
                                ui.label(
                                    RichText::new("Avg Percentile")
                                        .size(Theme::SIZE_CAPTION)
                                        .color(Theme::TEXT_SECONDARY),
                                );
                            });
                        });

                    ui.add_space(12.0);

                    // Top 25% count
                    egui::Frame::none()
                        .fill(Theme::BG_SECONDARY)
                        .rounding(Theme::BADGE_ROUNDING)
                        .inner_margin(egui::Margin::symmetric(16.0, 8.0))
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                ui.label(
                                    RichText::new(format!("{}", top_25_count))
                                        .size(Theme::SIZE_SECTION)
                                        .strong()
                                        .color(Theme::SUCCESS),
                                );
                                ui.label(
                                    RichText::new("Top 25% Tests")
                                        .size(Theme::SIZE_CAPTION)
                                        .color(Theme::TEXT_SECONDARY),
                                );
                            });
                        });

                    ui.add_space(12.0);

                    // Total tests
                    egui::Frame::none()
                        .fill(Theme::BG_SECONDARY)
                        .rounding(Theme::BADGE_ROUNDING)
                        .inner_margin(egui::Margin::symmetric(16.0, 8.0))
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                ui.label(
                                    RichText::new(format!("{}", total))
                                        .size(Theme::SIZE_SECTION)
                                        .strong()
                                        .color(Theme::TEXT_PRIMARY),
                                );
                                ui.label(
                                    RichText::new("Total Tests")
                                        .size(Theme::SIZE_CAPTION)
                                        .color(Theme::TEXT_SECONDARY),
                                );
                            });
                        });
                });
            });
    }

    fn show_rankings_grid(ui: &mut Ui, percentile_ranks: &[PercentileRank]) {
        if percentile_ranks.is_empty() {
            return;
        }

        egui::Frame::none()
            .fill(Theme::BG_CARD)
            .stroke(egui::Stroke::new(1.0, Theme::BORDER))
            .rounding(Theme::CARD_ROUNDING)
            .inner_margin(12.0)
            .show(ui, |ui| {
                ui.set_min_width(600.0);

                ui.label(
                    RichText::new("Your Rankings")
                        .size(Theme::SIZE_BODY)
                        .strong()
                        .color(Theme::TEXT_PRIMARY),
                );
                ui.add_space(8.0);

                // Grid of rankings
                egui::Grid::new("rankings_grid")
                    .num_columns(4)
                    .spacing([8.0, 4.0])
                    .show(ui, |ui| {
                        for (idx, rank) in percentile_ranks.iter().enumerate() {
                            let top_percent = 100.0 - rank.percentile_rank;
                            let color = Self::get_percentile_color(top_percent);

                            egui::Frame::none()
                                .fill(color.linear_multiply(0.3))
                                .rounding(Theme::BADGE_ROUNDING)
                                .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                                .show(ui, |ui| {
                                    ui.set_min_width(140.0);
                                    ui.vertical(|ui| {
                                        // Test name (truncated)
                                        let display_name = if rank.test_name.len() > 18 {
                                            format!("{}...", &rank.test_name[..18])
                                        } else {
                                            rank.test_name.clone()
                                        };
                                        ui.label(
                                            RichText::new(display_name)
                                                .size(Theme::SIZE_CAPTION)
                                                .color(Theme::TEXT_SECONDARY),
                                        );

                                        // Percentile
                                        ui.label(
                                            RichText::new(format!("Top {:.0}%", top_percent))
                                                .size(Theme::SIZE_BODY)
                                                .strong()
                                                .color(color),
                                        );
                                    });
                                });

                            if (idx + 1) % 4 == 0 {
                                ui.end_row();
                            }
                        }
                    });
            });
    }

    fn show_distribution_chart(
        ui: &mut Ui,
        stat: &TestStatistics,
        percentile: Option<&PercentileRank>,
    ) {
        egui::Frame::none()
            .fill(Theme::BG_CARD)
            .stroke(egui::Stroke::new(1.0, Theme::BORDER))
            .rounding(Theme::CARD_ROUNDING)
            .inner_margin(12.0)
            .show(ui, |ui| {
                ui.set_min_width(600.0);

                // Header
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new(&stat.test_name)
                                .size(Theme::SIZE_BODY)
                                .strong()
                                .color(Theme::TEXT_PRIMARY),
                        );
                        ui.label(
                            RichText::new(format!(
                                "{} samples | {}",
                                stat.sample_count, stat.unit
                            ))
                            .size(Theme::SIZE_CAPTION)
                            .color(Theme::TEXT_SECONDARY),
                        );
                    });

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if let Some(pct) = percentile {
                            let top_percent = 100.0 - pct.percentile_rank;
                            let color = Self::get_percentile_color(top_percent);

                            ui.vertical(|ui| {
                                ui.label(
                                    RichText::new(format!("Top {:.0}%", top_percent))
                                        .size(Theme::SIZE_BODY)
                                        .strong()
                                        .color(color),
                                );
                                ui.label(
                                    RichText::new(format!(
                                        "Your score: {:.2} {}",
                                        pct.user_value, stat.unit
                                    ))
                                    .size(Theme::SIZE_CAPTION)
                                    .color(Theme::TEXT_SECONDARY),
                                );
                            });
                        }
                    });
                });

                ui.add_space(4.0);

                // Stats row
                ui.horizontal(|ui| {
                    Self::stat_badge(ui, "Min", stat.min_value);
                    Self::stat_badge(ui, "Median", stat.p50);
                    Self::stat_badge(ui, "Max", stat.max_value);

                    if let Some(pct) = percentile {
                        if !pct.is_higher_better {
                            egui::Frame::none()
                                .fill(Color32::from_rgb(234, 179, 8).linear_multiply(0.2))
                                .rounding(Theme::BADGE_ROUNDING)
                                .inner_margin(egui::Margin::symmetric(8.0, 2.0))
                                .show(ui, |ui| {
                                    ui.label(
                                        RichText::new("Lower is better")
                                            .size(Theme::SIZE_CAPTION)
                                            .color(Color32::from_rgb(234, 179, 8)),
                                    );
                                });
                        }
                    }
                });

                ui.add_space(8.0);

                // Distribution chart using egui_plot
                if !stat.histogram_buckets.is_empty() {
                    let user_value = percentile.map(|p| p.user_value);

                    // Create bars from histogram buckets
                    let bars: Vec<Bar> = stat
                        .histogram_buckets
                        .iter()
                        .enumerate()
                        .map(|(idx, bucket)| {
                            let is_user_bucket = user_value.map_or(false, |uv| {
                                uv >= bucket.bucket_start && uv < bucket.bucket_end
                            });

                            let color = if is_user_bucket {
                                Color32::from_rgb(46, 204, 113) // Green for user's bucket
                            } else {
                                Color32::from_rgb(59, 130, 246) // Blue for others
                            };

                            Bar::new(idx as f64, bucket.count as f64)
                                .width(0.8)
                                .fill(color)
                        })
                        .collect();

                    let chart = BarChart::new(bars);

                    Plot::new(format!("histogram_{}", stat.test_id))
                        .height(120.0)
                        .show_axes([false, true])
                        .show_grid([false, true])
                        .allow_drag(false)
                        .allow_zoom(false)
                        .allow_scroll(false)
                        .legend(Legend::default())
                        .show(ui, |plot_ui| {
                            plot_ui.bar_chart(chart);

                            // Add median line
                            let median_bucket_idx = stat
                                .histogram_buckets
                                .iter()
                                .position(|b| stat.p50 >= b.bucket_start && stat.p50 < b.bucket_end)
                                .unwrap_or(0);
                            plot_ui.vline(
                                VLine::new(median_bucket_idx as f64)
                                    .color(Color32::from_rgb(241, 196, 15))
                                    .style(egui_plot::LineStyle::dashed_loose()),
                            );

                            // Add user value line
                            if let Some(uv) = user_value {
                                if let Some(user_bucket_idx) = stat
                                    .histogram_buckets
                                    .iter()
                                    .position(|b| uv >= b.bucket_start && uv < b.bucket_end)
                                {
                                    plot_ui.vline(
                                        VLine::new(user_bucket_idx as f64)
                                            .color(Color32::from_rgb(46, 204, 113))
                                            .width(2.0),
                                    );
                                }
                            }
                        });

                    // Legend
                    ui.horizontal(|ui| {
                        ui.add_space(ui.available_width() / 4.0);

                        // Community
                        let (rect, _) = ui.allocate_exact_size(egui::vec2(12.0, 12.0), egui::Sense::hover());
                        ui.painter().rect_filled(rect, 2.0, Color32::from_rgb(59, 130, 246));
                        ui.label(
                            RichText::new("Community")
                                .size(Theme::SIZE_CAPTION)
                                .color(Theme::TEXT_SECONDARY),
                        );

                        ui.add_space(12.0);

                        if user_value.is_some() {
                            let (rect, _) = ui.allocate_exact_size(egui::vec2(12.0, 12.0), egui::Sense::hover());
                            ui.painter().rect_filled(rect, 2.0, Color32::from_rgb(46, 204, 113));
                            ui.label(
                                RichText::new("Your Result")
                                    .size(Theme::SIZE_CAPTION)
                                    .color(Theme::TEXT_SECONDARY),
                            );

                            ui.add_space(12.0);
                        }

                        // Median
                        let (rect, _) = ui.allocate_exact_size(egui::vec2(12.0, 2.0), egui::Sense::hover());
                        ui.painter().rect_filled(rect, 0.0, Color32::from_rgb(241, 196, 15));
                        ui.label(
                            RichText::new("Median")
                                .size(Theme::SIZE_CAPTION)
                                .color(Theme::TEXT_SECONDARY),
                        );
                    });
                }
            });
    }

    fn stat_badge(ui: &mut Ui, label: &str, value: f64) {
        egui::Frame::none()
            .fill(Theme::BG_SECONDARY)
            .rounding(Theme::BADGE_ROUNDING)
            .inner_margin(egui::Margin::symmetric(8.0, 2.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(format!("{}:", label))
                            .size(Theme::SIZE_CAPTION)
                            .color(Theme::TEXT_SECONDARY),
                    );
                    ui.label(
                        RichText::new(Self::format_value(value))
                            .size(Theme::SIZE_CAPTION)
                            .color(Theme::TEXT_PRIMARY),
                    );
                });
            });
    }

    fn format_value(value: f64) -> String {
        if value.abs() >= 10000.0 {
            format!("{:.0}", value)
        } else if value.abs() >= 100.0 {
            format!("{:.1}", value)
        } else if value.abs() >= 1.0 {
            format!("{:.2}", value)
        } else {
            format!("{:.3}", value)
        }
    }

    fn get_percentile_color(top_percent: f64) -> Color32 {
        if top_percent <= 10.0 {
            Color32::from_rgb(34, 197, 94) // green-500
        } else if top_percent <= 25.0 {
            Color32::from_rgb(74, 222, 128) // green-400
        } else if top_percent <= 50.0 {
            Color32::from_rgb(234, 179, 8) // yellow-500
        } else if top_percent <= 75.0 {
            Color32::from_rgb(249, 115, 22) // orange-500
        } else {
            Color32::from_rgb(239, 68, 68) // red-500
        }
    }
}
