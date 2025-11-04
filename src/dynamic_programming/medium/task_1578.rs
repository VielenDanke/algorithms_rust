pub struct Solution;

impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let colors_bytes = colors.as_bytes();

        let (mut max, mut total, mut result) = (needed_time[0], needed_time[0], 0);

        for i in 0..colors_bytes.len() {
            if i > 0 {
                if colors_bytes[(i - 1)] == colors_bytes[i] {
                    max = max.max(needed_time[i]);
                    total += needed_time[i];
                } else {
                    result += total - max;
                    total = needed_time[i];
                    max = needed_time[i];
                }
            }
        }
        result + total - max
    }
}