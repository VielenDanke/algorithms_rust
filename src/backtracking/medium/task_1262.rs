pub struct Solution;

impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        fn backtrack(nums: &Vec<i32>, dp: &mut Vec<Vec<i32>>, idx: usize, remainder: usize) -> i32 {
            if idx == nums.len() {
                if remainder == 0 {
                    0
                } else {
                    i32::MIN
                }
            } else {
                if dp[idx][remainder] != i32::MIN {
                    dp[idx][remainder]
                } else {
                    let not_take = backtrack(nums, dp, idx + 1, remainder);
                    let new_remainder = ((nums[idx] + remainder as i32) % 3) as usize;
                    let take = nums[idx] + backtrack(nums, dp, idx + 1, new_remainder);

                    dp[idx][remainder] = take.max(not_take);

                    dp[idx][remainder]
                }
            }
        }
        let mut dp = vec![vec![i32::MIN; 3]; nums.len()];

        backtrack(&nums, &mut dp, 0, 0)
    }
}
