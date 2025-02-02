pub struct Solution;

impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        let mut left = 0;
        let mut right = 1;

        if nums.len() <= 1 {
            true
        } else {
            while right < nums.len() {
                if nums[left] % 2 == 0 && nums[right] % 2 == 0 {
                    return false;
                } else if nums[left] % 2 != 0 && nums[right] % 2 != 0 {
                    return false;
                }
                left += 1;
                right += 1;
            }
            true
        }
    }

    pub fn is_array_special_declarative(nums: Vec<i32>) -> bool {
        !nums
            .windows(2)
            .any(|w| (w[0] % 2 == 0 && w[1] % 2 == 0) || (w[0] % 2 != 0 && w[1] % 2 != 0))
    }
}
