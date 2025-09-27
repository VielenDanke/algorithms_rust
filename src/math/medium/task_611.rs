pub struct Solution;

impl Solution {
    pub fn triangle_number_brute_force(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let mut counter = 0;

        for i in 0..n {
            for j in i+1..n {
                for k in j+1..n {
                    if (nums[i] + nums[j] > nums[k]) && (nums[j] + nums[k] > nums[i]) && (nums[k] + nums[i] > nums[j]) {
                        counter += 1;
                    }
                }
            }
        }
        counter
    }

    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let mut counter = 0;

        nums.sort_unstable();

        for i in (2..=n-1).rev() {
            let (mut left_idx, mut right_idx) = (0, i - 1);
            while left_idx < right_idx {
                if nums[right_idx] + nums[left_idx] > nums[i] {
                    counter += (right_idx - left_idx) as i32;
                    right_idx -= 1;
                } else {
                    left_idx += 1;
                }
            }
        }
        counter
    }
}