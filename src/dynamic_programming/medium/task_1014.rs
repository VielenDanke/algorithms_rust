pub struct Solution;

impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        let (mut result, mut current) = (0, 0);

        for num in values.into_iter() {
            result = result.max(current + num);
            current = current.max(num) - 1;
        }
        result
    }

    pub fn max_score_sightseeing_pair_brute_force(values: Vec<i32>) -> i32 {
        let mut best_result = 0;
        let mut dp = vec![0; values.len()];

        for i in 0..values.len() {
            for j in 0..i {
                dp[i] = i32::max(dp[i], values[j] + values[i] + j as i32 - i as i32);
                best_result = best_result.max(dp[i]);
            }
        }
        best_result
    }
}