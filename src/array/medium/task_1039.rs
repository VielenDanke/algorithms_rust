pub struct Solution;

impl Solution {
    pub fn min_score_triangulation(polygon_values: Vec<i32>) -> i32 {
        let vertex_count = polygon_values.len();
        let mut min_score = vec![vec![0; vertex_count]; vertex_count];

        for gap in 2..vertex_count {
            for start in 0..(vertex_count - gap) {
                let end = start + gap;
                let mut current_min_score = 1 << 30;

                for mid in (start + 1)..end {
                    let triangle_score =
                        min_score[start][mid]
                            + min_score[mid][end]
                            + polygon_values[start] * polygon_values[mid] * polygon_values[end];
                    if triangle_score < current_min_score {
                        current_min_score = triangle_score;
                    }
                }
                min_score[start][end] = current_min_score;
            }
        }

        min_score[0][vertex_count - 1]
    }
}