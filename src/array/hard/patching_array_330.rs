pub struct Solution;

impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let nums = nums.into_iter().map(|x| x as i64).collect::<Vec<_>>();

        let mut index = 0;
        let mut count = 0;
        let mut value_need = 1_i64;
        while value_need <= n as i64 {
            if index < nums.len() && value_need >= nums[index] {
                value_need += nums[index];
                index += 1;
            } else {
                value_need += value_need;
                count += 1;
            }
        }
        count
    }
}
