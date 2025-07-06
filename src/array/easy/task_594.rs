/*
We define a harmonious array as an array where the difference between its maximum value and its minimum value is exactly 1.

Given an integer array nums, return the length of its longest harmonious subsequence among all its possible subsequences.
 */
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut counter = HashMap::new();

        nums.iter()
            .for_each(|&x| *counter.entry(x).or_insert(0i32) += 1);

        let mut max = 0;

        for (&k, &v) in counter.iter() {
            let left = *counter.get(&(k - 1)).unwrap_or(&0);
            let right = *counter.get(&(k + 1)).unwrap_or(&0);
            if left != 0 || right != 0 {
                max = max.max(v + left).max(v + right);
            }
        }

        max
    }

    pub fn find_lhs_shorter(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums.into_iter()
            .scan((0, 0, 0, 0), |(a, b, c, d), n| {
                if *c == n {
                    *d += 1;
                } else {
                    *a = *c;
                    *b = *d;
                    *c = n;
                    *d = 1;
                }
                Some((*a + 1 == *c && *b != 0).then_some(*b + *d))
            })
            .flatten()
            .max()
            .unwrap_or(0)
    }
}
