use egui::{Align, Layout, Response, RichText, Ui, Widget};

use crate::models::Rating;
use crate::ui::widgets::ProgressBar;
use crate::ui::Theme;

/// Score card widget showing a score with rating
pub struct ScoreCard<'a> {
    title: &'a str,
    score: u32,
    max_score: u32,
    rating: Rating,
}

impl<'a> ScoreCard<'a> {
    pub fn new(title: &'a str, score: u32, max_score: u32, rating: Rating) -> Self {
        Self {
            title,
            score,
            max_score,
            rating,
        }
    }
}

impl<'a> Widget for ScoreCard<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let percentage = if self.max_score > 0 {
            self.score as f32 / self.max_score as f32
        } else {
            0.0
        };

        egui::Frame::none()
            .fill(Theme::BG_CARD)
            .stroke(egui::Stroke::new(1.0, Theme::BORDER))
            .rounding(8.0)
            .inner_margin(16.0)
            .show(ui, |ui| {
                ui.set_min_width(180.0);

                ui.with_layout(Layout::top_down(Align::Center), |ui| {
                    // Title
                    ui.label(RichText::new(self.title).color(Theme::TEXT_SECONDARY).size(14.0));

                    ui.add_space(8.0);

                    // Score
                    ui.label(
                        RichText::new(format!("{}", self.score))
                            .color(Theme::rating_color(&self.rating))
                            .size(36.0)
                            .strong(),
                    );

                    // Max score
                    ui.label(
                        RichText::new(format!("/ {}", self.max_score))
                            .color(Theme::TEXT_SECONDARY)
                            .size(14.0),
                    );

                    ui.add_space(8.0);

                    // Progress bar
                    ui.add(ProgressBar::new(percentage).rating(self.rating).height(8.0));

                    ui.add_space(8.0);

                    // Rating badge
                    egui::Frame::none()
                        .fill(Theme::rating_bg_color(&self.rating))
                        .rounding(4.0)
                        .inner_margin(egui::Margin::symmetric(12.0, 4.0))
                        .show(ui, |ui| {
                            ui.label(
                                RichText::new(self.rating.label())
                                    .color(Theme::rating_color(&self.rating))
                                    .strong(),
                            );
                        });
                });
            })
            .response
    }
}

/// Large score card for overall results
pub struct LargeScoreCard<'a> {
    title: &'a str,
    score: u32,
    max_score: u32,
    rating: Rating,
}

impl<'a> LargeScoreCard<'a> {
    pub fn new(title: &'a str, score: u32, max_score: u32, rating: Rating) -> Self {
        Self {
            title,
            score,
            max_score,
            rating,
        }
    }
}

impl<'a> Widget for LargeScoreCard<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let percentage = if self.max_score > 0 {
            self.score as f32 / self.max_score as f32
        } else {
            0.0
        };

        egui::Frame::none()
            .fill(Theme::BG_CARD)
            .stroke(egui::Stroke::new(1.0, Theme::BORDER))
            .rounding(12.0)
            .inner_margin(24.0)
            .show(ui, |ui| {
                ui.set_min_width(280.0);

                ui.with_layout(Layout::top_down(Align::Center), |ui| {
                    // Title
                    ui.label(RichText::new(self.title).color(Theme::TEXT_SECONDARY).size(18.0));

                    ui.add_space(12.0);

                    // Score
                    ui.label(
                        RichText::new(format!("{}", self.score))
                            .color(Theme::rating_color(&self.rating))
                            .size(56.0)
                            .strong(),
                    );

                    // Max score
                    ui.label(
                        RichText::new(format!("/ {}", self.max_score))
                            .color(Theme::TEXT_SECONDARY)
                            .size(18.0),
                    );

                    ui.add_space(12.0);

                    // Progress bar
                    ui.add(ProgressBar::new(percentage).rating(self.rating).height(12.0));

                    ui.add_space(12.0);

                    // Rating badge
                    egui::Frame::none()
                        .fill(Theme::rating_bg_color(&self.rating))
                        .rounding(6.0)
                        .inner_margin(egui::Margin::symmetric(16.0, 8.0))
                        .show(ui, |ui| {
                            ui.label(
                                RichText::new(self.rating.label())
                                    .color(Theme::rating_color(&self.rating))
                                    .size(18.0)
                                    .strong(),
                            );
                        });
                });
            })
            .response
    }
}
