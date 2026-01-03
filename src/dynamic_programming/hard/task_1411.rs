const MOD: i64 = 1_000_000_007;

pub struct Solution;

impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        let (mut a, mut b) = (6i64, 6i64);

        for _ in 2..=n {
            let new_a = (2 * a + 2 * b) % MOD;
            let new_b = (2 * a + 3 * b) % MOD;
            a = new_a;
            b = new_b;
        }

        ((a + b) % MOD) as i32
    }

    pub fn num_of_ways_recursion(n: i32) -> i32 {
        let n = n as usize;

        // Initialize DP table: dp[index][prev1][prev2][prev3]
        // Using fixed-size arrays for the colors is faster than nested Vectors.
        // We use Option<i32> where None represents -1 (unvisited).
        let mut dp = vec![[[[None; 4]; 4]; 4]; n];

        // Define the recursive function
        fn solve(
            i: usize,
            n: usize,
            prev1: usize,
            prev2: usize,
            prev3: usize,
            dp: &mut Vec<[[[Option<i32>; 4]; 4]; 4]>,
        ) -> i32 {
            if i == n {
                return 1;
            }

            // Check memoization
            if let Some(val) = dp[i][prev1][prev2][prev3] {
                return val;
            }

            let mut ans: i64 = 0;

            // Iterate through colors (1 to 3)
            for col1 in 1..=3 {
                if col1 == prev1 {
                    continue;
                }
                for col2 in 1..=3 {
                    if col2 == prev2 || col2 == col1 {
                        continue;
                    }
                    for col3 in 1..=3 {
                        if col3 == prev3 || col3 == col2 {
                            continue;
                        }

                        ans = (ans + solve(i + 1, n, col1, col2, col3, dp) as i64) % MOD;
                    }
                }
            }

            let res = ans as i32;
            dp[i][prev1][prev2][prev3] = Some(res);
            res
        }

        // Start recursion with row 0 and "0" as previous colors (no constraint)
        solve(0, n, 0, 0, 0, &mut dp)
    }
}
