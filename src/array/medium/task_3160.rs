pub struct Solution;

impl Solution {
    pub fn query_results(limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = queries.len();
        let mut result = Vec::with_capacity(n);
        let mut color_map = std::collections::HashMap::new();
        let mut ball_map = std::collections::HashMap::new();

        for query in queries {
            let ball = query[0];
            let color = query[1];

            if let Some(&prev_color) = ball_map.get(&ball) {
                if let Some(count) = color_map.get_mut(&prev_color) {
                    *count -= 1;
                    if *count == 0 {
                        color_map.remove(&prev_color);
                    }
                }
            }

            ball_map.insert(ball, color);
            *color_map.entry(color).or_insert(0) += 1;

            result.push(color_map.len() as i32);
        }

        result
    }
}