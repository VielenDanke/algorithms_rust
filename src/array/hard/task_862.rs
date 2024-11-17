use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut window = 1;
        let mut idx = 0;
        while window <= n {
            while idx + window <= n {
                let mut temp_sum = 0;
                let slice = &nums[idx..idx + window];

                for &v in slice.iter() {
                    temp_sum += v;
                }
                if temp_sum >= k {
                    return slice.len() as i32;
                }
                idx += 1;
            }
            idx = 0;
            window += 1;
        }
        -1
    }

    pub fn shortest_subarray_fast(nums: Vec<i32>, k: i32) -> i32 {
        let mut min = i32::MAX;
        let mut sums: Vec<i64> = vec![0; nums.len() + 1];

        for (i, num) in nums.iter().enumerate() {
            if nums[i] >= k {
                return 1;
            }
            sums[i + 1] = sums[i] + *num as i64;
        }
        let mut dequeue: VecDeque<usize> = VecDeque::with_capacity(nums.len());

        for i in 0..=nums.len() {
            while dequeue.front().is_some_and(|&j| (sums[i] - sums[j]) >= k as i64) {
                min = std::cmp::min(min, (i - dequeue.pop_front().unwrap()) as i32);
            }
            while dequeue.back().is_some_and(|&j| sums[j] >= sums[i]) {
                dequeue.pop_back();
            }
            dequeue.push_back(i);
        }
        match min {
            i32::MAX => -1,
            _ => min,
        }
    }
}