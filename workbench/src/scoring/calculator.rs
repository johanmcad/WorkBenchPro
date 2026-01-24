use crate::models::{CategoryResults, CategoryScore, CategoryScores, Rating, Scores};

pub struct ScoreCalculator;

impl ScoreCalculator {
    pub fn calculate(results: &CategoryResults) -> Scores {
        let project_ops = Self::calculate_category_score(&results.project_operations, 2500);
        let build_perf = Self::calculate_category_score(&results.build_performance, 2500);
        let responsiveness = Self::calculate_category_score(&results.responsiveness, 2500);
        let graphics = results.graphics.as_ref().map(|g| Self::calculate_category_score(g, 2500));

        let overall = project_ops.score + build_perf.score + responsiveness.score
            + graphics.as_ref().map(|g| g.score).unwrap_or(0);

        let overall_max = 2500 * 3 + if graphics.is_some() { 2500 } else { 0 };

        let percentage = if overall_max > 0 {
            (overall as f64 / overall_max as f64) * 100.0
        } else {
            0.0
        };

        Scores {
            overall,
            overall_max,
            rating: Rating::from_percentage(percentage),
            categories: CategoryScores {
                project_operations: project_ops,
                build_performance: build_perf,
                responsiveness,
                graphics,
            },
        }
    }

    fn calculate_category_score(
        results: &[crate::models::TestResult],
        _max_category_score: u32,
    ) -> CategoryScore {
        let score: u32 = results.iter().map(|r| r.score).sum();
        let max_score: u32 = results.iter().map(|r| r.max_score).sum();

        CategoryScore::new(score, max_score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{TestDetails, TestResult};

    fn make_test_result(score: u32, max_score: u32) -> TestResult {
        TestResult {
            test_id: "test".to_string(),
            name: "Test".to_string(),
            description: "Test".to_string(),
            value: 100.0,
            unit: "units".to_string(),
            score,
            max_score,
            details: TestDetails::default(),
        }
    }

    #[test]
    fn test_calculate_scores() {
        let results = CategoryResults {
            project_operations: vec![
                make_test_result(400, 500),
                make_test_result(500, 600),
            ],
            build_performance: vec![
                make_test_result(300, 600),
            ],
            responsiveness: vec![
                make_test_result(350, 400),
            ],
            graphics: None,
        };

        let scores = ScoreCalculator::calculate(&results);

        assert_eq!(scores.categories.project_operations.score, 900);
        assert_eq!(scores.categories.project_operations.max_score, 1100);
        assert_eq!(scores.categories.build_performance.score, 300);
        assert_eq!(scores.categories.responsiveness.score, 350);
    }
}
