pub struct Solution;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        fn backtrack(nums: &Vec<i32>, target: i32, mut sum: i32, total_sum: i32, idx: usize, memo: &mut Vec<Vec<Option<i32>>>) -> i32 {
            if memo[idx][(total_sum + sum) as usize].is_some() {
                return memo[idx][(total_sum + sum) as usize].unwrap();
            }
            if idx == nums.len() {
                if target == sum {
                    1
                } else {
                    0
                }
            } else {
                let mut result = backtrack(nums, target, sum + nums[idx] * -1, total_sum, idx + 1, memo);
                result += backtrack(nums, target, sum + nums[idx], total_sum, idx + 1, memo);
                memo[idx][(total_sum + sum) as usize] = Some(result);
                result
            }
        }
        let total_sum = nums.iter().sum::<i32>();
        let mut memo = vec![vec![None; (total_sum * 2 + 1) as usize]; nums.len() + 1];
        backtrack(&nums, target, 0, total_sum, 0, &mut memo)
    }
}
