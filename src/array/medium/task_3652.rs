pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>, strategy: Vec<i32>, k: i32) -> i64 {
        let n = prices.len();
        let k = k as usize;
        let mid = k / 2;

        // 1. Calculate Base Profit
        // Summing up strategy[i] * prices[i] for the entire array
        let base_profit: i64 = (0..n).map(|i| prices[i] as i64 * strategy[i] as i64).sum::<i64>();

        // Edge case safety (though constraints usually say k <= n)
        if k > n {
            return base_profit;
        }

        // 2. Calculate the Delta for the Initial Window [0 ... k-1]
        let mut current_delta: i64 = 0;
        for i in 0..k {
            let original_val = (prices[i] as i64) * (strategy[i] as i64);

            if i < mid {
                // First Half: Force Hold (0)
                // Change = New(0) - Original
                current_delta += 0 - original_val;
            } else {
                // Second Half: Force Sell (1)
                // Change = New(price) - Original
                current_delta += (prices[i] as i64) - original_val;
            }
        }

        // Initialize max_delta. Start at 0 because we can choose not to modify.
        let mut max_delta = 0.max(current_delta);

        // 3. Slide the window
        // Loop runs from index 1 to n-k (inclusive)
        for i in 1..=(n - k) {
            let leaving_idx = i - 1;
            let mid_idx = i - 1 + mid;
            let entering_idx = i + k - 1;

            // A. Handle Leaving Element (Remove 'Hold' effect)
            // Previous delta was (0 - original). To remove it: subtract (0 - original) => add original.
            current_delta += (prices[leaving_idx] as i64) * (strategy[leaving_idx] as i64);

            // B. Handle Middle Element (Shift Sell -> Hold)
            // Net change: (0 - original) - (price - original) = -price
            current_delta -= prices[mid_idx] as i64;

            // C. Handle Entering Element (Add 'Sell' effect)
            // New delta contribution: (price - original)
            current_delta += (prices[entering_idx] as i64) - ((prices[entering_idx] as i64) * (strategy[entering_idx] as i64));

            // Update Max
            if current_delta > max_delta {
                max_delta = current_delta;
            }
        }

        base_profit + max_delta
    }
}