pub struct Solution;

impl Solution {
    pub fn minimize_max(mut nums: Vec<i32>, mut p: i32) -> i32 {
        fn is_able_to_form_pairs(nums: &Vec<i32>, middle: i32, p: i32) -> bool {
            let mut count = 0;
            let mut idx = 0;

            while idx < nums.len() - 1 && count < p {
                if nums[idx + 1] - nums[idx] <= middle {
                    count += 1;
                    idx += 2;
                } else {
                    idx += 1;
                }
            }
            count >= p
        }

        nums.sort_unstable();

        let n = nums.len();

        let (mut left, mut right) = (0, nums[n - 1] - nums[0]);

        while left < right {
            let middle = left + (right - left) / 2;

            if is_able_to_form_pairs(&nums, middle, p) {
                right = middle;
            } else {
                left = middle + 1;
            }
        }
        left
    }
}