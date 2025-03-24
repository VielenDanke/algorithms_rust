pub struct Solution;

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut counter = 0;
        let n = nums.len();

        let mut i = 0;

        while i + 2 < n {
            if nums[i] == 0 {
                counter += 1;
                nums[i] = 1;
                if nums[i + 1] == 0 {
                    nums[i + 1] = 1;
                } else {
                    nums[i + 1] = 0;
                }
                if nums[i + 2] == 0 {
                    nums[i + 2] = 1;
                } else {
                    nums[i + 2] = 0;
                }
            }
            i += 1;
        }
        if nums[n - 1] == 0 || nums[n - 2] == 0 {
            return -1;
        }
        counter
    }
}