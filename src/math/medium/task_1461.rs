use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let mut visited = HashSet::new();

        let s_bytes = s.as_bytes();
        let n = s_bytes.len() - k as usize;

        for i in 0..=n {
            let sub_bytes = &s_bytes[i..i + (k as usize)];
            visited.insert(sub_bytes);
        }
        visited.len() == (1 << k) as usize
    }
}
