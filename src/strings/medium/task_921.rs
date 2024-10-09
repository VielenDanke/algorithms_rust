use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut stack = VecDeque::new();
        let left_par = '(';
        let mut counter = 0;

        for ch in s.chars() {
            if ch == left_par {
                stack.push_back(ch);
            } else {
                if stack.is_empty() {
                    counter += 1;
                } else {
                    stack.pop_back();
                }
            }
        }
        (stack.len() + counter) as i32
    }

    pub fn min_add_to_make_valid_no_stack(s: String) -> i32 {
        let mut unclosed = 0;
        let mut unopened = 0;
        let left_par = '(';

        for ch in s.chars() {
            if ch == left_par {
                unclosed += 1;
            } else if unclosed > 0 {
                unclosed -= 1;
            } else {
                unopened += 1;
            }
        }
        unclosed + unopened
    }
}