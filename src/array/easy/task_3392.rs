pub struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>) -> i32 {
        let mut counter = 0;

        for i in 0..nums.len() {
            if i + 2 == nums.len() {
                break;
            }
            if (nums[i] + nums[i + 2]) as f64 == nums[i + 1] as f64 / 2f64 {
                counter += 1;
            }
        }
        counter
    }
}
