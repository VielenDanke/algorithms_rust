pub struct Solution;

impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let (mut total, mut odd, mut count, mut idx) = (0, 0, 0, 0);

        let is_odd = |x| x % 2 == 1;

        for &num in nums.iter() {
            if is_odd(num) {
                odd += 1;
                count = 0;
            }

            while odd == k {
                if is_odd(nums[idx]) {
                    odd -= 1;
                }
                count += 1;
                idx += 1;
            }

            total += count;
        }

        total
    }

    pub fn number_of_subarrays_prefix_sum(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();

        let mut prefix_sum = vec![0; n + 1];

        prefix_sum[0] = 1;

        let (mut result, mut t) = (0, 0);

        for &num in nums.iter() {
            t += num & 1; // if num % 2 == 0 { 0 } else { 1 }

            if t - k >= 0 {
                result += prefix_sum[(t - k) as usize];
            }

            prefix_sum[t as usize] += 1;
        }

        result
    }

    pub fn number_of_subarrays_tle(nums: Vec<i32>, k: i32) -> i32 {
        let mut window = 1;
        let mut result = 0;

        while window <= nums.len() {
            let mut idx = 0;

            while idx + window <= nums.len() {
                let sub_array = &nums[idx..idx + window];

                let mut odds = 0;

                for &num in sub_array.iter() {
                    if num % 2 != 0 {
                        odds += 1;
                    }
                }

                if odds == k {
                    result += 1;
                }

                idx += 1;
            }
            window += 1;
        }

        result
    }
}
