use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        let size = 3;

        let mut counter = HashMap::new();

        let s_bytes = s.as_bytes();

        let n = s_bytes.len();

        for i in 0..n {
            let mut index = i;
            while index < n && s_bytes[index] == s_bytes[i] {
                *counter.entry(String::from_utf8_lossy(&s_bytes[i..index + 1]).to_string())
                    .or_insert(0) += 1;
                index += 1;
            }
        }

        let mut max_len = -1;

        for (k, &v) in counter.iter() {
            if v >= size {
                max_len = max_len.max(k.len() as i32);
            }
        }

        max_len
    }
}