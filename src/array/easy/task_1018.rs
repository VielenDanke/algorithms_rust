pub struct Solution;

impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        let mut result = Vec::with_capacity(nums.len());
        let mut current_remainder = 0;

        for num in nums {
            current_remainder = (current_remainder * 2 + num) % 5;

            result.push(current_remainder == 0);
        }

        result
    }
}