use std::cmp::max;
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn maximum_profit(prices: Vec<i32>, k: i32) -> i64 {
        let n = prices.len();
        if n < 2 || k == 0 {
            return 0;
        }

        // dp[t][i] = max profit starting from day i with t transactions left
        // We only need dp[t-1] and dp[t], but keep full table for clarity
        let mut dp = vec![vec![0i64; n + 1]; k as usize + 1];

        for t in 1..=k {
            let mut best_plus: i64 = i64::MIN;
            let mut best_minus: i64 = i64::MIN;

            // iterate days backwards
            for i in (0..=n - 2).rev() {
                let p_next = prices[i + 1] as i64;

                // update helpers using dp[t-1]
                best_plus = max(best_plus, p_next + dp[t as usize - 1][i + 2]);
                best_minus = max(best_minus, -p_next + dp[t as usize - 1][i + 2]);

                let p = prices[i] as i64;

                // option 1: skip day i
                let skip = dp[t as usize][i + 1];

                // option 2: start a transaction at day i
                let take = max(
                    -p + best_plus, // normal: buy at i
                    p + best_minus, // short: sell at i
                );

                dp[t as usize][i] = max(skip, take);
            }
        }

        dp[k as usize][0]
    }

    pub fn maximum_profit_dfs(prices: Vec<i32>, k: i32) -> i64 {
        fn dfs(
            memo: &mut HashMap<(usize, i32), i64>,
            prices: &Vec<i32>,
            idx: usize,
            k: i32,
        ) -> i64 {
            if k == 0 || idx >= prices.len() {
                return 0;
            }

            let key = (idx, k);

            if let Some(v) = memo.get(&key) {
                return *v;
            }

            let mut best = dfs(memo, prices, idx + 1, k);

            let current_price = prices[idx] as i64;

            for i in idx + 1..prices.len() {
                let next_price = prices[i] as i64;
                let current_profit =
                    (next_price - current_price).abs() + dfs(memo, prices, i + 1, k - 1);
                best = best.max(current_profit);
            }

            memo.insert(key, best);

            best
        }
        dfs(&mut HashMap::new(), &prices, 0, k)
    }
}
