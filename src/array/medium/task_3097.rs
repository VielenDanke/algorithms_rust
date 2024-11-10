pub struct Solution;

impl Solution {
    fn update_counts(counter_arr: &mut Vec<i32>, mut val: i32, delta: i32) {
        let mut idx = 0;

        while val != 0 {
            if val % 2 == 1 {
                counter_arr[idx] += delta;
            }
            val /= 2;
            idx += 1;
        }
    }

    fn counts_to_num(counter_arr: &Vec<i32>) -> i32 {
        let (mut result, mut multiplayer) = (0, 1);

        for i in 0..30 {
            if counter_arr[i] > 0 {
                result += multiplayer;
            }
            multiplayer *= 2;
        }
        result
    }

    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        if k == 0 {
            1
        } else {
            let (mut left, mut result, mut counter_arr) = (0, usize::MAX, vec![0; 30]);

            for right in 0..nums.len() {
                Self::update_counts(&mut counter_arr, nums[right], 1);

                while Self::counts_to_num(&counter_arr) >= k {
                    result = result.min(right - left + 1);
                    Self::update_counts(&mut counter_arr, nums[left], -1);
                    left += 1;
                }
            }
            if result == usize::MAX {
                -1
            } else {
                result as i32
            }
        }
    }
}