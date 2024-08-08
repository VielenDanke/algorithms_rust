use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn kth_distinct(arr: Vec<String>, mut k: i32) -> String {
        let seen = arr.iter()
            .fold(HashMap::new(), |mut acc, s| {
                acc.entry(s).and_modify(|v| *v = false).or_insert(true);
                acc
            });

        arr.iter()
            .filter(|s| *seen.get(s).unwrap())
            .nth((k - 1) as usize)
            .unwrap_or(&String::new())
            .to_owned()
    }
}