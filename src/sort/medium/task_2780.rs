use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn minimum_index(nums: Vec<i32>) -> i32 {
        let mut counter_right: HashMap<i32, usize> = HashMap::new();

        for &num in nums.iter() {
            *counter_right.entry(num).or_default() += 1;
        }
        let mut counter_left: HashMap<i32, usize> = HashMap::new();

        for (idx, &num) in nums.iter().enumerate() {
            *counter_left.entry(num).or_default() += 1;
            *counter_right.entry(num).or_default() -= 1;

            if *counter_left.entry(num).or_default() * 2 > idx + 1 && *counter_right.entry(num).or_default() * 2 > nums.len() - idx - 1 {
                return idx as i32;
            }
        }

        -1
    }
}