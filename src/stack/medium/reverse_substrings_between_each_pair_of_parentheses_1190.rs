use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut stack = VecDeque::new();
        let mut result = Vec::new();

        for ch in s.chars() {
            if ch == '(' {
                stack.push_back(result.len());
            } else if ch == ')' {
                let mut start_idx = stack.pop_back().unwrap();
                let mut slice = &mut result[start_idx..];
                slice.reverse();
            } else {
                result.push(ch as u8);
            }
        }

        String::from_utf8(result).unwrap()
    }
}
