use crate::array::create_prefix_sum;

pub struct Solution;

impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let prefix_sum = create_prefix_sum(&nums);

        let mut mod_groups = vec![0; k as usize];

        mod_groups[0] = 1;

        let mut result = 0;

        for &sum in prefix_sum.iter() {
            let mut current_mod = sum % k;
            if current_mod < 0 {
                current_mod += k;
            }
            result += mod_groups[current_mod as usize];
            mod_groups[current_mod as usize] += 1;
        }
        result
    }

    pub fn subarrays_div_by_k_no_prefix_sum_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum = 0;
        let mut count = 0;
        let mut frequency = vec![0; k as usize];

        frequency[0] = 1;

        for num in nums {
            sum += num;
            let mod_k = ((sum % k) + k) % k;
            count += frequency[mod_k as usize];
            frequency[mod_k as usize] += 1;
        }

        count
    }

    pub fn subarrays_div_by_k_tle(nums: Vec<i32>, k: i32) -> i32 {
        let prefix_sum = create_prefix_sum(&nums);
        let mut window = 0;
        let mut result = 0;

        while window <= nums.len() {
            for i in 0..nums.len() {
                if i + window < nums.len() {
                    if i > 0 {
                        if (prefix_sum[i + window] - prefix_sum[i - 1]) % k == 0 {
                            result += 1;
                        }
                    } else {
                        if prefix_sum[i + window] % k == 0 {
                            result += 1;
                        }
                    }
                }
            }
            window += 1;
        }
        result
    }
}
