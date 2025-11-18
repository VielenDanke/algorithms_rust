pub struct Solution;

impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut prev_idx = 0;
        for (i, &num) in nums.iter().enumerate() {
            if num == 1 {
                prev_idx = i;
                break;
            }
        }
        for i in prev_idx+1..nums.len() {
            if nums[i] == 1 {
                if i - prev_idx <= k as usize {
                    return false;
                }
                prev_idx = i;
            }
        }
        true
    }
}