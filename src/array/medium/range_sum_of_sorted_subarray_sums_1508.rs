use std::collections::BTreeSet;

const MODULUS: i64 = (10e8 + 7f64) as i64;

pub struct Solution;

impl Solution {
    pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        let mut bh = BTreeSet::new();

        let m = n as usize;

        for i in 0..m {
            bh.insert((nums[i], i));
        }

        let (mut result, modulus) = (0, (10e8 + 7f64) as i32);

        for i in 1..=right {
            if let Some(mut first_entry) = bh.pop_first() {
                if i >= left {
                    result = (result + first_entry.0) % modulus;
                }
                if first_entry.1 < m - 1 {
                    first_entry.1 += 1;
                    first_entry.0 += nums[first_entry.1];
                    bh.insert(first_entry);
                }
            }
        }

        result
    }

    pub fn range_sum_sliding_window(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        let mut window = 1;

        let mut temp = Vec::new();

        let m = n as usize;

        while window <= nums.len() {
            for i in 0..m {
                if i + window > nums.len() {
                    break;
                }
                temp.push(nums[i..i+window].into_iter().map(|v| *v as i64).sum::<i64>());
            }
            window += 1;
        }
        temp.sort_unstable();

        let mut sum = 0;

        for &v in temp[(left - 1) as usize..right as usize].iter() {
            sum = (sum + v) % MODULUS;
        }

        sum as i32
    }
}