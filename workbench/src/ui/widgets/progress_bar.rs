use egui::{Color32, Rect, Response, Sense, Ui, Vec2, Widget};

use crate::models::Rating;
use crate::ui::Theme;

/// Custom progress bar widget
pub struct ProgressBar {
    progress: f32,
    height: f32,
    rating: Option<Rating>,
}

impl ProgressBar {
    pub fn new(progress: f32) -> Self {
        Self {
            progress: progress.clamp(0.0, 1.0),
            height: 8.0,
            rating: None,
        }
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn rating(mut self, rating: Rating) -> Self {
        self.rating = Some(rating);
        self
    }
}

impl Widget for ProgressBar {
    fn ui(self, ui: &mut Ui) -> Response {
        let desired_size = Vec2::new(ui.available_width(), self.height);
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            // Background
            painter.rect_filled(rect, self.height / 2.0, Theme::BORDER);

            // Fill
            let fill_width = rect.width() * self.progress;
            let fill_rect = Rect::from_min_size(rect.min, Vec2::new(fill_width, self.height));

            let fill_color = self
                .rating
                .map(|r| Theme::rating_color(&r))
                .unwrap_or(Theme::ACCENT);

            painter.rect_filled(fill_rect, self.height / 2.0, fill_color);
        }

        response
    }
}

/// Animated progress bar for running benchmarks
pub struct AnimatedProgressBar {
    progress: f32,
    height: f32,
}

impl AnimatedProgressBar {
    pub fn new(progress: f32) -> Self {
        Self {
            progress: progress.clamp(0.0, 1.0),
            height: 8.0,
        }
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }
}

impl Widget for AnimatedProgressBar {
    fn ui(self, ui: &mut Ui) -> Response {
        let desired_size = Vec2::new(ui.available_width(), self.height);
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            // Background
            painter.rect_filled(rect, self.height / 2.0, Theme::BORDER);

            // Fill with gradient effect
            let fill_width = rect.width() * self.progress;
            let fill_rect = Rect::from_min_size(rect.min, Vec2::new(fill_width, self.height));

            painter.rect_filled(fill_rect, self.height / 2.0, Theme::ACCENT);

            // Request repaint for animation
            ui.ctx().request_repaint();
        }

        response
    }
}
