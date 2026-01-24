use egui::{Align, Layout, Response, RichText, Ui, Vec2, Widget};

use crate::models::Rating;
use crate::ui::widgets::ProgressBar;
use crate::ui::Theme;

/// Score card widget matching 05-ui-design.md spec
/// - Size: 200x140px
/// - Border radius: 8px
/// - Border: 1px #e2e8f0
/// - Background: white
/// - Score: 36px bold centered
/// - Progress bar: 8px height
/// - Rating badge at bottom
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
            .rounding(Theme::CARD_ROUNDING)
            .inner_margin(12.0)
            .show(ui, |ui| {
                ui.set_min_size(Vec2::new(Theme::CARD_WIDTH, Theme::CARD_HEIGHT));
                ui.set_max_width(Theme::CARD_WIDTH);

                ui.with_layout(Layout::top_down(Align::Center), |ui| {
                    // Title
                    ui.label(
                        RichText::new(self.title)
                            .color(Theme::TEXT_SECONDARY)
                            .size(Theme::SIZE_CAPTION),
                    );

                    ui.add_space(6.0);

                    // Score
                    ui.label(
                        RichText::new(format!("{}", self.score))
                            .color(Theme::rating_color(&self.rating))
                            .size(Theme::SIZE_SCORE)
                            .strong(),
                    );

                    // Max score
                    ui.label(
                        RichText::new(format!("/ {}", self.max_score))
                            .color(Theme::TEXT_SECONDARY)
                            .size(Theme::SIZE_CAPTION),
                    );

                    ui.add_space(8.0);

                    // Progress bar (8px height per spec)
                    ui.add(
                        ProgressBar::new(percentage)
                            .rating(self.rating)
                            .height(Theme::PROGRESS_HEIGHT)
                            .width(Theme::CARD_WIDTH - 24.0),
                    );

                    ui.add_space(8.0);

                    // Rating badge (padding: 8px x 4px, border radius: 4px)
                    egui::Frame::none()
                        .fill(Theme::rating_bg_color(&self.rating))
                        .rounding(Theme::BADGE_ROUNDING)
                        .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                        .show(ui, |ui| {
                            ui.label(
                                RichText::new(self.rating.label())
                                    .color(Theme::rating_color(&self.rating))
                                    .size(Theme::SIZE_CAPTION)
                                    .strong(),
                            );
                        });
                });
            })
            .response
    }
}

/// Large score card for overall results display
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
            .rounding(Theme::CARD_ROUNDING_LARGE)
            .inner_margin(24.0)
            .show(ui, |ui| {
                ui.set_min_width(300.0);

                ui.with_layout(Layout::top_down(Align::Center), |ui| {
                    // Title
                    ui.label(
                        RichText::new(self.title)
                            .color(Theme::TEXT_SECONDARY)
                            .size(Theme::SIZE_SECTION),
                    );

                    ui.add_space(12.0);

                    // Score (larger for overall)
                    ui.label(
                        RichText::new(format!("{}", self.score))
                            .color(Theme::rating_color(&self.rating))
                            .size(Theme::SIZE_SCORE_LARGE)
                            .strong(),
                    );

                    // Max score
                    ui.label(
                        RichText::new(format!("/ {}", self.max_score))
                            .color(Theme::TEXT_SECONDARY)
                            .size(Theme::SIZE_CARD),
                    );

                    ui.add_space(16.0);

                    // Progress bar
                    ui.add(
                        ProgressBar::new(percentage)
                            .rating(self.rating)
                            .height(12.0)
                            .width(260.0),
                    );

                    ui.add_space(16.0);

                    // Rating badge (larger)
                    egui::Frame::none()
                        .fill(Theme::rating_bg_color(&self.rating))
                        .rounding(6.0)
                        .inner_margin(egui::Margin::symmetric(16.0, 8.0))
                        .show(ui, |ui| {
                            ui.label(
                                RichText::new(self.rating.label())
                                    .color(Theme::rating_color(&self.rating))
                                    .size(Theme::SIZE_CARD)
                                    .strong(),
                            );
                        });
                });
            })
            .response
    }
}
