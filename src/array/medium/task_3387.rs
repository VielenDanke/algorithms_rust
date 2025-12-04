pub struct Solution;

impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let mut max_sum = i64::MIN;
        let k = k as usize;
        let mut k_sum = vec![i64::MAX / 2; k];
        let mut prefix_sum = 0;
        k_sum[k-1] = 0;
        for i in 0..n {
            prefix_sum += nums[i] as i64;
            max_sum = max_sum.max(prefix_sum - k_sum[i%k]);
            k_sum[i%k] = k_sum[i%k].min(prefix_sum);
        }
        max_sum
    }
}