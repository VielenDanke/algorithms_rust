pub struct Solution;

impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut prefix_sum = vec![0i64; n];

        prefix_sum[0] = nums[0] as i64;

        for i in 1..n {
            prefix_sum[i] = prefix_sum[i - 1] + nums[i] as i64;
        }
        let mut result = 0;

        for i in 0..n-1 {
            if prefix_sum[i] >= prefix_sum[n - 1] - prefix_sum[i] {
                result += 1;
            }
        }

        result
    }

    pub fn ways_to_split_array_optimized(nums: Vec<i32>) -> i32 {
        let (mut left_sum, mut right_sum, n) = (0i64, 0i64, nums.len());

        for &num in nums.iter() {
            right_sum += num as i64;
        }

        let mut result = 0;

        for i in 0..n - 1 {
            left_sum += nums[i] as i64;
            right_sum -= nums[i] as i64;

            if left_sum >= right_sum {
                result += 1;
            }
        }

        result
    }
}