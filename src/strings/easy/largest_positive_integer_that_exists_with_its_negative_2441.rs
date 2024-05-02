use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let mut storage: HashSet<i32> = HashSet::from_iter(nums.clone());
        let mut result = None;
        for v in &nums {
            if *v > 0 && storage.contains(&(*v * -1)) {
                result = match result {
                    None => Some(*v),
                    Some(current_result) => Some(current_result.max(*v)),
                }
            }
        }
        match result {
            None => -1,
            Some(v) => v,
        }
    }
}
