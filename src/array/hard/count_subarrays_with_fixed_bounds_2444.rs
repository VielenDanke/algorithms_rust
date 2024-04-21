use std::cmp::{max, min};

pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
    let (mut result, mut bad_idx, mut left_idx, mut right_idx) = (0i64, -1i64, -1i64, -1i64);

    for (i, num) in nums.iter().enumerate() {
        let idx = i as i64;
        if !(&min_k <= num && num <= &max_k) {
            bad_idx = idx;
        }
        if num == &min_k {
            left_idx = idx;
        }
        if num == &max_k {
            right_idx = idx;
        }
        result += max(0, min(left_idx, right_idx) - bad_idx);
    }
    result
}

pub fn count_subarrays_second(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
    let (mut result, mut max_i, mut min_i, mut left_i) = (0i64, -1i64, -1i64, 0i64);
    for right_i in 0..nums.len() {
        let x = nums[right_i];
        if x < min_k || x > max_k {
            left_i = (right_i + 1) as i64;
            continue;
        }
        if x == max_k {
            max_i = right_i as i64;
        }
        if x == min_k {
            min_i = right_i as i64;
        }
        result += i64::max((i64::min(max_i, min_i) - left_i + 1), 0);
    }
    result
}
