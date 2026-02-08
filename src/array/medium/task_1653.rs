use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let mut stack = VecDeque::new();
        let s_bytes = s.as_bytes();
        let n = s_bytes.len();
        let mut delete_count = 0;

        for i in 0..n {
            if !stack.is_empty() && *stack.back().unwrap() == b'b' && s_bytes[i] == b'a' {
                stack.pop_back();
                delete_count += 1;
            } else {
                stack.push_back(s_bytes[i]);
            }
        }

        delete_count
    }
}