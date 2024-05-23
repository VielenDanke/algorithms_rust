use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;
        Self::backtrack(&nums, &mut HashMap::new(), 0, k, &mut result);
        result
    }

    fn backtrack(nums: &Vec<i32>, visited: &mut HashMap<i32, i32>, start: usize, k: i32, result: &mut i32) {
        if !visited.is_empty() {
            *result += 1;
        }
        for i in start..nums.len() {
            let (left, right) = (nums[i] - k, nums[i] + k);
            if !visited.contains_key(&left) && !visited.contains_key(&right) {
                *visited.entry(nums[i]).or_insert(0) += 1;
                Self::backtrack(nums, visited, i + 1, k, result);
                let val = visited.get_mut(&nums[i]).unwrap();
                *val -= 1;
                if *val == 0 {
                    visited.remove(&nums[i]);
                }
            }
        }
    }
}
