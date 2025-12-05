pub struct Solution;

impl Solution {
    pub fn count_partitions(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        if n == 0 {
            return 0;
        }

        let mut prefix_sum = vec![0; n];

        prefix_sum[0] = nums[0];

        for i in 1..n {
            prefix_sum[i] = prefix_sum[i - 1] + nums[i];
        }

        let (mut left, right, mut result) = (0, n - 1, 0);

        while left < right {
            if ((prefix_sum[right] - prefix_sum[left]) - prefix_sum[left]) % 2 == 0 {
                result += 1;
            }

            left += 1;
        }

        result
    }
}