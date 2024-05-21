use std::collections::VecDeque;

pub struct Solution;

impl Solution {

    pub fn subset_xor_sum_short(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |acc, v| acc | v) << (nums.len() - 1)
    }

    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        Self::backtrack(&nums, &mut VecDeque::new(), 0, &mut result);
        result
    }

    fn backtrack(nums: &Vec<i32>, temp: &mut VecDeque<i32>, start: usize, result: &mut i32) {
        *result += Self::calculate_xor(temp);
        for i in start..nums.len() {
            temp.push_back(*nums.get(i).unwrap());
            Self::backtrack(nums, temp, i + 1, result);
            temp.pop_back();
        }
    }

    fn calculate_xor(temp: &VecDeque<i32>) -> i32 {
        if temp.is_empty() {
            return 0;
        }
        let mut current = None;
        for num in temp.iter() {
            if current.is_none() {
                current = Some(*num);
            } else {
                current = Some(current.take().unwrap() ^ num);
            }
        }
        current.unwrap()
    }
}
