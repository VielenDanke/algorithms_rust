use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut last_occurrence = HashMap::new();
        
        for (i, c) in s.chars().enumerate() {
            last_occurrence.insert(c, i);
        }
        let mut result = Vec::new();
        let mut start = 0;
        let mut end = 0;

        for (i, c) in s.chars().enumerate() {
            if let Some(&last) = last_occurrence.get(&c) {
                end = end.max(last);
            }
            if i == end {
                result.push((end - start + 1) as i32);
                start = i + 1;
            }
        }

        result
    }
}