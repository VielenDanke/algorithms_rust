pub struct Solution;

impl Solution {
    pub fn count_hills_valleys(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 3 {
            return 0;
        }

        let mut count = 0;
        let mut left_val = nums[0];

        for i in 1..n - 1 {
            if nums[i] == left_val {
                continue;
            }
            let mut right_val_found = false;
            let mut right_val = 0;

            for j in i + 1..n {
                if nums[j] != nums[i] {
                    right_val = nums[j];
                    right_val_found = true;
                    break;
                }
            }

            if !right_val_found {
                continue;
            }

            if nums[i] > left_val && nums[i] > right_val {
                count += 1;
            } else if nums[i] < left_val && nums[i] < right_val {
                count += 1;
            }

            left_val = nums[i];
        }

        count
    }
}
