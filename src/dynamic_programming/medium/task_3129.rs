use std::collections::HashMap;

pub struct Solution;

impl Solution {

    fn sherlockAndAnagrams(s: &str) -> i32 {
        let s_bytes = s.as_bytes();
        let n = s.len();
        let mut m: HashMap<Vec<u8>, i32> = HashMap::new();
        for i in 0..n {
            for j in i + 1..n {
                let mut key = s_bytes[i..j].to_vec();
                key.sort_unstable();
                *m.entry(key).or_default() += 1;
            }
        }
        let mut total = 0;

        for (_, val) in m {
            total += (val * (val - 1)) / 2
        }

        total
    }
    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        let z_idx = zero as usize;
        let o_idx = one as usize;
        let lim = limit as usize;
        let mod_val = 1_000_000_007i64;

        // dp[i][j][0] = stable arrays with i zeros, j ones, ending in 0
        // dp[i][j][1] = stable arrays with i zeros, j ones, ending in 1
        // Using a 3D-like structure with Vec<Vec<[i64; 2]>> for efficiency
        let mut dp = vec![vec![[0i64; 2]; o_idx + 1]; z_idx + 1];

        // Base cases: single runs within the limit
        // Sequences like "0", "00", "000" are valid if length <= limit
        for i in 1..=z_idx.min(lim) {
            dp[i][0][0] = 1;
        }
        for j in 1..=o_idx.min(lim) {
            dp[0][j][1] = 1;
        }

        for i in 1..=z_idx {
            for j in 1..=o_idx {
                // --- Calculate dp[i][j][0] ---
                // We can append a 0 to any valid array ending in 0 or 1
                dp[i][j][0] = (dp[i - 1][j][0] + dp[i - 1][j][1]) % mod_val;

                if i > lim {
                    // Subtract sequences that would result in (limit + 1) zeros.
                    // These are sequences that ended in a 1 at (i - limit - 1)
                    // followed by (limit) zeros, where we just added the (limit + 1)-th zero.
                    let sub = dp[i - lim - 1][j][1];
                    dp[i][j][0] = (dp[i][j][0] - sub + mod_val) % mod_val;
                }

                // --- Calculate dp[i][j][1] ---
                // We can append a 1 to any valid array ending in 0 or 1
                dp[i][j][1] = (dp[i][j - 1][0] + dp[i][j - 1][1]) % mod_val;

                if j > lim {
                    // Subtract sequences that would result in (limit + 1) ones.
                    let sub = dp[i][j - lim - 1][0];
                    dp[i][j][1] = (dp[i][j][1] - sub + mod_val) % mod_val;
                }
            }
        }

        ((dp[z_idx][o_idx][0] + dp[z_idx][o_idx][1]) % mod_val) as i32
    }
}