use egui::{Color32, FontFamily, FontId, Style, TextStyle, Visuals};

use crate::models::Rating;

/// Application theme and colors
pub struct Theme;

impl Theme {
    // Primary colors
    pub const BG_PRIMARY: Color32 = Color32::from_rgb(248, 250, 252);
    pub const BG_CARD: Color32 = Color32::WHITE;
    pub const BG_DARK: Color32 = Color32::from_rgb(26, 26, 46);

    // Text colors
    pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(30, 41, 59);
    pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(100, 116, 139);

    // Accent colors
    pub const ACCENT: Color32 = Color32::from_rgb(15, 52, 96);
    pub const ACCENT_HOVER: Color32 = Color32::from_rgb(26, 74, 122);

    // Rating colors
    pub const SCORE_EXCELLENT: Color32 = Color32::from_rgb(16, 185, 129);
    pub const SCORE_GOOD: Color32 = Color32::from_rgb(59, 130, 246);
    pub const SCORE_ACCEPTABLE: Color32 = Color32::from_rgb(245, 158, 11);
    pub const SCORE_POOR: Color32 = Color32::from_rgb(239, 68, 68);
    pub const SCORE_INADEQUATE: Color32 = Color32::from_rgb(127, 29, 29);

    // Neutral colors
    pub const BORDER: Color32 = Color32::from_rgb(226, 232, 240);

    pub fn rating_color(rating: &Rating) -> Color32 {
        match rating {
            Rating::Excellent => Self::SCORE_EXCELLENT,
            Rating::Good => Self::SCORE_GOOD,
            Rating::Acceptable => Self::SCORE_ACCEPTABLE,
            Rating::Poor => Self::SCORE_POOR,
            Rating::Inadequate => Self::SCORE_INADEQUATE,
        }
    }

    pub fn rating_bg_color(rating: &Rating) -> Color32 {
        let color = Self::rating_color(rating);
        Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 38) // 15% opacity
    }

    pub fn apply(ctx: &egui::Context) {
        let mut style = Style::default();

        // Configure text styles
        style.text_styles = [
            (TextStyle::Heading, FontId::new(28.0, FontFamily::Proportional)),
            (TextStyle::Name("Section".into()), FontId::new(20.0, FontFamily::Proportional)),
            (TextStyle::Name("Card".into()), FontId::new(16.0, FontFamily::Proportional)),
            (TextStyle::Body, FontId::new(14.0, FontFamily::Proportional)),
            (TextStyle::Small, FontId::new(12.0, FontFamily::Proportional)),
            (TextStyle::Monospace, FontId::new(14.0, FontFamily::Monospace)),
            (TextStyle::Button, FontId::new(14.0, FontFamily::Proportional)),
        ]
        .into();

        // Configure visuals
        let mut visuals = Visuals::light();
        visuals.panel_fill = Self::BG_PRIMARY;
        visuals.window_fill = Self::BG_CARD;
        visuals.widgets.noninteractive.bg_fill = Self::BG_CARD;
        visuals.widgets.inactive.bg_fill = Self::BG_CARD;
        visuals.widgets.hovered.bg_fill = Color32::from_rgb(241, 245, 249);
        visuals.widgets.active.bg_fill = Self::ACCENT;
        visuals.selection.bg_fill = Self::ACCENT;

        style.visuals = visuals;

        // Apply spacing
        style.spacing.item_spacing = egui::vec2(8.0, 8.0);
        style.spacing.window_margin = egui::Margin::same(16.0);
        style.spacing.button_padding = egui::vec2(12.0, 6.0);

        ctx.set_style(style);
    }
}
