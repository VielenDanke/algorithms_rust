use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut sum = 0;
        let mut map = HashMap::new();
        map.insert(0, -1i32);

        for (i, &num) in nums.iter().enumerate() {
            sum += num;
            if k != 0 {
                sum %= k;
            }
            if let Some(&prev_index) = map.get(&sum) {
                if (i as i32) - prev_index >= 2 {
                    return true;
                }
            } else {
                map.insert(sum, i as i32);
            }
        }

        false
    }
}
