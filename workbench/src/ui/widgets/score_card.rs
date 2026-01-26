use egui::{Align, Layout, Response, RichText, Ui, Vec2, Widget};

use crate::ui::Theme;

/// Category summary card widget for displaying benchmark category information
/// Replaces the score-based cards with raw value focused display
pub struct CategorySummaryCard<'a> {
    title: &'a str,
    test_count: usize,
    summary: &'a str,
}

impl<'a> CategorySummaryCard<'a> {
    pub fn new(title: &'a str, test_count: usize, summary: &'a str) -> Self {
        Self {
            title,
            test_count,
            summary,
        }
    }
}

impl<'a> Widget for CategorySummaryCard<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        egui::Frame::none()
            .fill(Theme::BG_CARD)
            .stroke(egui::Stroke::new(1.0, Theme::BORDER))
            .rounding(Theme::CARD_ROUNDING)
            .inner_margin(12.0)
            .show(ui, |ui| {
                ui.set_min_size(Vec2::new(180.0, 100.0));
                ui.set_max_width(200.0);

                ui.with_layout(Layout::top_down(Align::Center), |ui| {
                    // Category title
                    ui.label(
                        RichText::new(self.title)
                            .color(Theme::TEXT_PRIMARY)
                            .size(Theme::SIZE_BODY)
                            .strong(),
                    );

                    ui.add_space(8.0);

                    // Test count
                    ui.label(
                        RichText::new(format!("{} tests", self.test_count))
                            .color(Theme::ACCENT)
                            .size(Theme::SIZE_SECTION)
                            .strong(),
                    );

                    ui.add_space(4.0);

                    // Summary (e.g., key metric or status)
                    ui.label(
                        RichText::new(self.summary)
                            .color(Theme::TEXT_SECONDARY)
                            .size(Theme::SIZE_CAPTION),
                    );
                });
            })
            .response
    }
}

/// Machine info header card
pub struct MachineInfoCard<'a> {
    machine_name: &'a str,
    timestamp: &'a str,
    total_tests: usize,
}

impl<'a> MachineInfoCard<'a> {
    pub fn new(machine_name: &'a str, timestamp: &'a str, total_tests: usize) -> Self {
        Self {
            machine_name,
            timestamp,
            total_tests,
        }
    }
}

impl<'a> Widget for MachineInfoCard<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        egui::Frame::none()
            .fill(Theme::BG_CARD)
            .stroke(egui::Stroke::new(1.0, Theme::BORDER))
            .rounding(Theme::CARD_ROUNDING_LARGE)
            .inner_margin(16.0)
            .show(ui, |ui| {
                ui.set_min_width(300.0);

                ui.with_layout(Layout::top_down(Align::Center), |ui| {
                    // Machine name
                    ui.label(
                        RichText::new(self.machine_name)
                            .color(Theme::ACCENT)
                            .size(Theme::SIZE_SECTION)
                            .strong(),
                    );

                    ui.add_space(4.0);

                    // Timestamp
                    ui.label(
                        RichText::new(self.timestamp)
                            .color(Theme::TEXT_SECONDARY)
                            .size(Theme::SIZE_BODY),
                    );

                    ui.add_space(8.0);

                    // Total tests completed badge
                    egui::Frame::none()
                        .fill(Theme::BG_SECONDARY)
                        .rounding(4.0)
                        .inner_margin(egui::Margin::symmetric(10.0, 4.0))
                        .show(ui, |ui| {
                            ui.label(
                                RichText::new(format!("{} tests completed", self.total_tests))
                                    .color(Theme::TEXT_PRIMARY)
                                    .size(Theme::SIZE_BODY),
                            );
                        });
                });
            })
            .response
    }
}

/// Compact test result row for displaying individual benchmark results
pub struct TestResultRow<'a> {
    name: &'a str,
    value: f64,
    unit: &'a str,
}

impl<'a> TestResultRow<'a> {
    pub fn new(name: &'a str, value: f64, unit: &'a str) -> Self {
        Self { name, value, unit }
    }
}

impl<'a> Widget for TestResultRow<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| {
            // Test name (left-aligned)
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                ui.label(
                    RichText::new(self.name)
                        .color(Theme::TEXT_PRIMARY)
                        .size(Theme::SIZE_BODY),
                );
            });

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                // Unit
                ui.label(
                    RichText::new(self.unit)
                        .color(Theme::TEXT_SECONDARY)
                        .size(Theme::SIZE_CAPTION),
                );

                // Value (formatted appropriately)
                let value_str = if self.value >= 1000.0 {
                    format!("{:.0}", self.value)
                } else if self.value >= 1.0 {
                    format!("{:.2}", self.value)
                } else {
                    format!("{:.3}", self.value)
                };

                ui.label(
                    RichText::new(value_str)
                        .color(Theme::ACCENT)
                        .size(Theme::SIZE_BODY)
                        .strong(),
                );
            });
        })
        .response
    }
}
