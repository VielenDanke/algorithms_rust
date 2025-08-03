pub struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut window = nums.len();
        let mut max = 0;
        let mut max_length = 0;

        while window > 0 {
            for i in 0..nums.len() {
                if i + window <= nums.len() {
                    let current_max = nums[i..i + window].iter().fold(nums[i], |acc, &x| acc & x);

                    if current_max > max && max_length < window {
                        max = current_max;
                        max_length = window;
                    }
                }
            }
            window -= 1;
        }
        max_length as i32
    }

    pub fn longest_subarray_shorter(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        if nums.len() == 1 {
            return 1;
        }
        let mut max_value = 0;
        let mut result = 0;

        for mut i in 0..nums.len() {
            let mut count = 0;
            let mut new_max_value = nums[i];
            let mut prev_value = nums[i];
            while i < nums.len() && nums[i] == prev_value {
                count += 1;
                i += 1;
            }
            if new_max_value > max_value {
                result = count;
                max_value = new_max_value;
            } else if new_max_value == max_value {
                result = result.max(count);
            }
        }
        result
    }

    pub fn longest_subarray_extending(nums: Vec<i32>) -> i32 {
        let mut max_val = 0;
        let mut ans = 0;
        let mut current_streak = 0;

        for num in nums {
            if max_val < num {
                max_val = num;
                ans = 0;
                current_streak = 0;
            }

            if max_val == num {
                current_streak += 1;
            } else {
                current_streak = 0;
            }

            ans = ans.max(current_streak);
        }

        ans
    }
}
