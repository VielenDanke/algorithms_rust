pub struct Solution;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        if strs.is_empty() {
            return 0;
        }

        let w = strs[0].len();
        let str_bytes: Vec<&[u8]> = strs.iter().map(|s| s.as_bytes()).collect();

        let mut dp = vec![1; w];

        for i in (0..w).rev() {
            'search: for j in i + 1..w {
                if str_bytes.iter().any(|row| row[i] > row[j]) {
                    continue 'search;
                }
                dp[i] = dp[i].max(1 + dp[j]);
            }
        }

        (w as i32) - dp.into_iter().max().unwrap_or(0)
    }
}
