use egui::{Color32, Ui};
use egui_plot::{Bar, BarChart, Plot};

use crate::models::{BenchmarkRun, Rating};
use crate::ui::Theme;

/// Score chart widget for displaying category scores as a bar chart
pub struct ScoreChart<'a> {
    run: &'a BenchmarkRun,
    comparison: Option<&'a BenchmarkRun>,
}

impl<'a> ScoreChart<'a> {
    pub fn new(run: &'a BenchmarkRun) -> Self {
        Self {
            run,
            comparison: None,
        }
    }

    pub fn with_comparison(mut self, other: &'a BenchmarkRun) -> Self {
        self.comparison = Some(other);
        self
    }

    pub fn show(self, ui: &mut Ui) {
        let categories = [
            ("Project Ops", self.run.scores.categories.project_operations.score as f64,
             self.run.scores.categories.project_operations.max_score as f64,
             &self.run.scores.categories.project_operations.rating),
            ("Build Perf", self.run.scores.categories.build_performance.score as f64,
             self.run.scores.categories.build_performance.max_score as f64,
             &self.run.scores.categories.build_performance.rating),
            ("Responsive", self.run.scores.categories.responsiveness.score as f64,
             self.run.scores.categories.responsiveness.max_score as f64,
             &self.run.scores.categories.responsiveness.rating),
        ];

        let mut bars_a: Vec<Bar> = Vec::new();

        for (idx, (name, score, max, rating)) in categories.iter().enumerate() {
            let percentage = (score / max) * 100.0;
            let color = Self::rating_to_color(rating);

            let bar = Bar::new(idx as f64, percentage)
                .name(*name)
                .fill(color)
                .width(0.35);

            bars_a.push(bar);
        }

        let chart_a = BarChart::new(bars_a)
            .name(&self.run.machine_name)
            .color(Color32::from_rgb(100, 180, 255));

        Plot::new("score_chart")
            .height(200.0)
            .show_axes(true)
            .show_grid(true)
            .allow_zoom(false)
            .allow_drag(false)
            .allow_scroll(false)
            .y_axis_label("Score %")
            .show(ui, |plot_ui| {
                plot_ui.bar_chart(chart_a);

                if let Some(other) = self.comparison {
                    let other_categories = [
                        other.scores.categories.project_operations.score as f64 /
                            other.scores.categories.project_operations.max_score as f64 * 100.0,
                        other.scores.categories.build_performance.score as f64 /
                            other.scores.categories.build_performance.max_score as f64 * 100.0,
                        other.scores.categories.responsiveness.score as f64 /
                            other.scores.categories.responsiveness.max_score as f64 * 100.0,
                    ];

                    let bars_b: Vec<Bar> = other_categories
                        .iter()
                        .enumerate()
                        .map(|(idx, &pct)| {
                            Bar::new(idx as f64 + 0.4, pct)
                                .fill(Color32::from_rgb(255, 180, 100))
                                .width(0.35)
                        })
                        .collect();

                    let chart_b = BarChart::new(bars_b)
                        .name(&other.machine_name)
                        .color(Color32::from_rgb(255, 180, 100));

                    plot_ui.bar_chart(chart_b);
                }
            });
    }

    fn rating_to_color(rating: &Rating) -> Color32 {
        Theme::rating_color(rating)
    }
}

/// Test results chart showing individual test scores
pub struct TestResultsChart<'a> {
    results: &'a [crate::models::TestResult],
    title: &'a str,
}

impl<'a> TestResultsChart<'a> {
    pub fn new(title: &'a str, results: &'a [crate::models::TestResult]) -> Self {
        Self { results, title }
    }

    pub fn show(self, ui: &mut Ui) {
        if self.results.is_empty() {
            return;
        }

        let bars: Vec<Bar> = self.results
            .iter()
            .enumerate()
            .map(|(idx, result)| {
                let percentage = result.score_percentage();
                let rating = Rating::from_percentage(percentage);
                let color = Theme::rating_color(&rating);

                Bar::new(idx as f64, percentage)
                    .name(&result.name)
                    .fill(color)
                    .width(0.7)
            })
            .collect();

        let chart = BarChart::new(bars).color(Theme::ACCENT);

        Plot::new(format!("test_chart_{}", self.title))
            .height(150.0)
            .show_axes(true)
            .show_grid(true)
            .allow_zoom(false)
            .allow_drag(false)
            .allow_scroll(false)
            .y_axis_label("Score %")
            .show(ui, |plot_ui| {
                plot_ui.bar_chart(chart);
            });
    }
}

/// Comparison delta chart showing differences between runs
pub struct DeltaChart<'a> {
    results_a: &'a [crate::models::TestResult],
    results_b: &'a [crate::models::TestResult],
}

impl<'a> DeltaChart<'a> {
    pub fn new(
        results_a: &'a [crate::models::TestResult],
        results_b: &'a [crate::models::TestResult],
    ) -> Self {
        Self { results_a, results_b }
    }

    pub fn show(self, ui: &mut Ui) {
        let mut bars: Vec<Bar> = Vec::new();

        for (idx, result_a) in self.results_a.iter().enumerate() {
            if let Some(result_b) = self.results_b.iter().find(|r| r.test_id == result_a.test_id) {
                let pct_a = result_a.score_percentage();
                let pct_b = result_b.score_percentage();
                let delta = pct_a - pct_b;

                let color = if delta > 0.0 {
                    Theme::SUCCESS
                } else if delta < 0.0 {
                    Theme::ERROR
                } else {
                    Theme::TEXT_SECONDARY
                };

                bars.push(
                    Bar::new(idx as f64, delta)
                        .name(&result_a.name)
                        .fill(color)
                        .width(0.7),
                );
            }
        }

        if bars.is_empty() {
            return;
        }

        let chart = BarChart::new(bars);

        Plot::new("delta_chart")
            .height(150.0)
            .show_axes(true)
            .show_grid(true)
            .allow_zoom(false)
            .allow_drag(false)
            .allow_scroll(false)
            .y_axis_label("Difference %")
            .include_y(0.0)
            .show(ui, |plot_ui| {
                plot_ui.bar_chart(chart);
            });
    }
}
