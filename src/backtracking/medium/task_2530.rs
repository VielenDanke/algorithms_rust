use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn max_kelements(mut nums: Vec<i32>, k: i32) -> i64 {
        fn backtrack(nums: &mut Vec<i32>, k: i32, total: i64) -> i64 {
            if k == 0 {
                return total;
            }
            let mut total_sum = 0;
            for i in 0..nums.len() {
                let current_num = nums[i];
                nums[i] = (current_num as f64 / 3.0).ceil() as i32;
                total_sum = total_sum.max(backtrack(nums, k - 1, total + current_num as i64));
                nums[i] = current_num;
            }
            total_sum
        }
        backtrack(&mut nums, k, 0)
    }

    pub fn max_kelements_order(nums: Vec<i32>, mut k: i32) -> i64 {
        let mut bh = BinaryHeap::new();
        let delimiter = 3.0;

        for &num in nums.iter() {
            bh.push(num);
        }
        let mut sum = 0i64;
        while k > 0 {
            let mut num = bh.pop().unwrap();
            sum += num as i64;
            num = (num as f64 / delimiter).ceil() as i32;
            bh.push(num);
            k -= 1;
        }
        sum
    }
}