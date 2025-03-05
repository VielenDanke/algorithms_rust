pub struct Solution;

impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let mut min_prefix_sum = i32::MAX;
        let mut max_prefix_sum = i32::MIN;
        let mut prefix_sum = 0;
        let mut max_abs_sum = 0;

        for &num in &nums {
            // Prefix sum from index 0 to i
            prefix_sum += num;

            // Minimum & Maximum prefix sum we have seen so far
            min_prefix_sum = min_prefix_sum.min(prefix_sum);
            max_prefix_sum = max_prefix_sum.max(prefix_sum);

            if prefix_sum >= 0 {
                // If the prefixSum is positive, we will get the difference between prefixSum & minPrefixSum
                max_abs_sum = max_abs_sum.max(prefix_sum.max(prefix_sum - min_prefix_sum));
            } else {
                // If the prefixSum is negative, we will get the absolute difference between prefixSum & maxPrefixSum
                max_abs_sum = max_abs_sum.max(prefix_sum.abs().max((prefix_sum - max_prefix_sum).abs()));
            }
        }

        max_abs_sum
    }
}
