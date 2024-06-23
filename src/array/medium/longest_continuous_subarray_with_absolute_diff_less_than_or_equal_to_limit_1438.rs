use std::collections::BTreeMap;

pub struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let (mut left, mut result) = (0, 0);

        let mut tree_map = BTreeMap::new();

        for right in 0..nums.len() {
            *tree_map.entry(nums[right]).or_insert(0) += 1;

            let (mut smallest, mut largest) = (*tree_map.first_key_value().unwrap().0, *tree_map.last_key_value().unwrap().0);

            if largest - smallest <= limit {
                result = result.max((right - left + 1) as i32);
            } else {
                while left <= right && (largest - smallest) > limit {
                    let num = tree_map.get_mut(&nums[left]).unwrap();

                    *num -= 1;

                    if *num == 0 {
                        tree_map.remove(&nums[left]);
                    }

                    left += 1;

                    (smallest, largest) = (*tree_map.first_key_value().unwrap().0, *tree_map.last_key_value().unwrap().0);
                }
            }
        }

        result
    }

    pub fn longest_subarray_tle(nums: Vec<i32>, limit: i32) -> i32 {
        let mut window = 1;
        let mut result = 0;

        while window <= nums.len() {
            let mut idx = 0;

            while idx + window <= nums.len() {
                let subarray = &nums[idx..idx + window];

                let mut max = -1 << 30;
                let mut min = 1 << 30;

                for &num in subarray.iter() {
                    max = max.max(num);
                    min = min.min(num);
                }

                if max - min <= limit {
                    result = result.max(subarray.len() as i32);
                }

                idx += 1;
            }

            window += 1;
        }

        result
    }
}
