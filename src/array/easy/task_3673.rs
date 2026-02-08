pub struct Solution;

impl Solution {
    pub fn is_trionic(nums: Vec<i32>) -> bool {
        let mut idx = 0;
        let n = nums.len();

        if n < 4 || nums[0] > nums[1] {
            return false;
        }

        while idx < n - 1 && nums[idx + 1] - nums[idx] > 0 {
            idx += 1;
        }

        if idx == n - 1 {
            return false;
        }

        while idx < n - 1 && nums[idx] - nums[idx + 1] > 0 {
            idx += 1;
        }

        if idx == n - 1 {
            return false;
        }

        while idx < n - 1 && nums[idx + 1] - nums[idx] > 0 {
            idx += 1;
        }

        if idx == n - 1 {
            true
        } else {
            false
        }
    }
}

