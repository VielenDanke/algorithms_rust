pub struct Solution;

impl Solution {
    pub fn triangular_sum(mut nums: Vec<i32>) -> i32 {
        // create arrays until n == 1
        // new_nums[i] = (nums[i] + nums[i + 1] % 10)
        // replace array

        while nums.len() > 1 {
            for i in 0..nums.len() - 1 {
                nums[i] = (nums[i] + nums[i + 1]) % 10;
            }
            nums.pop();
        }
        nums[0]
    }
}