use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums: BinaryHeap<_> = nums.into_iter().map(|num| Reverse(num as u64)).collect();

        let mut count = 0;
        while let (Some(Reverse(small)), Some(mut large)) = (
            nums.pop().filter(|&Reverse(num)| num < k as u64),
            nums.peek_mut(),
        ) {
            large.0 += small * 2;
            count += 1;
        }
        count
    }
}
