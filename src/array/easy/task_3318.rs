use itertools::Itertools;
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
        nums.windows(k as usize)
            .map(|w| {
                let mut m = HashMap::new();
                for &v in w.iter() {
                    *m.entry(v as i64).or_insert(0) += 1;
                }
                m.keys()
                    .map(|v| (-m.get(&v).unwrap() as i64, -v))
                    .sorted()
                    .into_iter()
                    .take(x as usize)
                    .map(|(f, v)| v * f)
                    .sum()
            })
            .collect()
    }
}
