use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut greater = HashSet::new();

        for &n in &nums {
            if n < k {
                return -1;
            }
            if n > k {
                greater.insert(n);
            }
        }

        greater.len() as i32
    }
}
