use egui::{Rect, Response, Sense, Ui, Vec2, Widget};

use crate::models::Rating;
use crate::ui::Theme;

/// Custom progress bar widget matching 05-ui-design.md spec
/// - Height: 8px (large) or 6px (small)
/// - Border radius: 4px
/// - Background: #e2e8f0
/// - Fill: rating color
pub struct ProgressBar {
    progress: f32,
    height: f32,
    width: Option<f32>,
    rating: Option<Rating>,
}

impl ProgressBar {
    pub fn new(progress: f32) -> Self {
        Self {
            progress: progress.clamp(0.0, 1.0),
            height: Theme::PROGRESS_HEIGHT,
            width: None,
            rating: None,
        }
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn small(mut self) -> Self {
        self.height = Theme::PROGRESS_HEIGHT_SMALL;
        self
    }

    pub fn rating(mut self, rating: Rating) -> Self {
        self.rating = Some(rating);
        self
    }
}

impl Widget for ProgressBar {
    fn ui(self, ui: &mut Ui) -> Response {
        let width = self.width.unwrap_or_else(|| ui.available_width());
        let desired_size = Vec2::new(width, self.height);
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let rounding = Theme::PROGRESS_ROUNDING;

            // Background
            painter.rect_filled(rect, rounding, Theme::BORDER);

            // Fill
            if self.progress > 0.0 {
                let fill_width = rect.width() * self.progress;
                let fill_rect = Rect::from_min_size(rect.min, Vec2::new(fill_width, self.height));

                let fill_color = self
                    .rating
                    .map(|r| Theme::rating_color(&r))
                    .unwrap_or(Theme::ACCENT);

                painter.rect_filled(fill_rect, rounding, fill_color);
            }
        }

        response
    }
}
