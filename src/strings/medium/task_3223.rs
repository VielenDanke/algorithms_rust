use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        // pick index i where i >= 1 && i <= n - 2
        // delete the closest to the left that s[j] == s[i]
        // delete the closest to the right that s[k] == s[i]
        let mut m = HashMap::new();

        for &b in s.as_bytes().iter() {
            *m.entry(b).or_insert(0) += 1;
        }
        s.len() as i32
            - m.into_iter()
                .map(|(_, frequency)| {
                    if frequency % 2 == 1 {
                        frequency - 1
                    } else {
                        frequency - 2
                    }
                })
                .sum::<i32>()
    }

    pub fn minimum_length_brute_force(s: String) -> i32 {
        // pick index i where i >= 1 && i <= n - 2
        // delete the closest to the left that s[j] == s[i]
        // delete the closest to the right that s[k] == s[i]
        let mut s_bytes = s.as_bytes().to_vec();

        let mut idx = 1;

        while !s_bytes.is_empty() && idx < s_bytes.len() {
            let mut deleted_index_left = None;
            let mut deleted_index_right = None;

            for j in (0..idx).rev() {
                if s_bytes[idx] == s_bytes[j] {
                    deleted_index_left = Some(j);
                    break;
                }
            }
            for k in (idx + 1)..s_bytes.len() {
                if s_bytes[idx] == s_bytes[k] {
                    deleted_index_right = Some(k);
                    break;
                }
            }
            if deleted_index_left.is_some() && deleted_index_right.is_some() {
                s_bytes[deleted_index_left.unwrap()] = b'$';
                s_bytes[deleted_index_right.unwrap()] = b'$';
            } else {
                idx += 1;
            }
        }
        s_bytes.into_iter().filter(|&b| b != b'$').count() as i32
    }
}
