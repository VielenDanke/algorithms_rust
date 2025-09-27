pub struct Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let n = triangle.len();
        let mut dp = vec![0i32; n + 1];

        for i in (0..n).rev() {
            let current = &triangle[i];

            for j in 0..current.len() {
                dp[j] = dp[j].min(dp[j + 1]) + current[j];
            }
        }
        dp[0]
    }
}