use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut temp = VecDeque::new();
        let mut result = 0;

        for num in nums {
            while !temp.is_empty() && *temp.back().unwrap() > num {
                temp.pop_back();
            }
            if num == 0 {
                continue;
            }
            if temp.is_empty() || *temp.back().unwrap() < num {
                result += 1;
                temp.push_back(num);
            }
        }

        result
    }
}