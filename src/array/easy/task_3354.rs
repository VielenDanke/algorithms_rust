pub struct Solution;

impl Solution {
    pub fn count_valid_selections_prefix_sum(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let sum: i32 = nums.iter().sum();
        let mut left = 0;
        let mut right = sum;
        for &x in nums.iter() {
            if x == 0 {
                if left - right >= 0 && left - right <= 1 {
                    result += 1;
                }
                if right - left >= 0 && right - left <= 1 {
                    result += 1;
                }
            } else {
                left += x;
                right -= x;
            }
        }
        result
    }

    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        fn process(mut nums: Vec<i32>, mut direction: bool, mut idx: i32) -> i32 {
            while idx >= 0 && (idx as usize) < nums.len() {
                if idx < 0 || (idx as usize) >= nums.len() {
                    break;
                }
                if nums[idx as usize] > 0 {
                    direction = !direction;
                    nums[idx as usize] -= 1;
                }
                if direction {
                    idx -= 1;
                } else {
                    idx += 1;
                }
            }
            if nums.iter().filter(|&&num| num != 0).count() as i32 == 0 {
                1
            } else {
                0
            }
        }

        let mut result = 0;

        for i in 0..nums.len() {
            if nums[i] == 0 {
                // start process
                result += process(nums.clone(), true, i as i32);
                result += process(nums.clone(), false, i as i32);
            }
        }

        result
    }
}
