pub struct Solution;

impl Solution {
    pub fn max_adjacent_distance(nums: Vec<i32>) -> i32 {
        let mut max = i32::MIN;
        let n = nums.len() - 1;
        for i in 0..n {
            max = max.max((nums[i] - nums[i + 1]).abs());
        }
        max.max((nums[0] - nums[n]).abs())
    }
}