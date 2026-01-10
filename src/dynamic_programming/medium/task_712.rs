pub struct Solution;

impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let s1_bytes = s1.as_bytes();
        let s2_bytes = s2.as_bytes();
        let n = s1_bytes.len();
        let m = s2_bytes.len();

        // dp[i][j] stores the minimum cost to equalise s1[..i] and s2[..j]
        let mut dp = vec![vec![0; m + 1]; n + 1];

        // Initialize first column (transforming s1 into empty string)
        for i in 1..=n {
            dp[i][0] = dp[i - 1][0] + s1_bytes[i - 1] as i32;
        }

        // Initialize first row (transforming s2 into empty string)
        for j in 1..=m {
            dp[0][j] = dp[0][j - 1] + s2_bytes[j - 1] as i32;
        }

        // Fill the DP table
        for i in 1..=n {
            for j in 1..=m {
                if s1_bytes[i - 1] == s2_bytes[j - 1] {
                    // Characters match, no deletion cost added relative to previous state
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    // Characters don't match, take the minimum of:
                    // 1. Deleting char from s1 (cost comes from dp[i-1][j])
                    // 2. Deleting char from s2 (cost comes from dp[i][j-1])
                    let delete_s1 = dp[i - 1][j] + s1_bytes[i - 1] as i32;
                    let delete_s2 = dp[i][j - 1] + s2_bytes[j - 1] as i32;
                    dp[i][j] = std::cmp::min(delete_s1, delete_s2);
                }
            }
        }

        dp[n][m]
    }
}
