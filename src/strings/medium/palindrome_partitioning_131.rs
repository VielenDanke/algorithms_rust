use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut result = Vec::new();
        Self::backtrack(s.as_bytes(), &mut VecDeque::new(), &mut result);
        result
    }

    fn backtrack(s_bytes: &[u8], temp: &mut VecDeque<String>, result: &mut Vec<Vec<String>>) {
        if s_bytes.len() == 0 {
            result.push(Vec::from(temp.as_slices().0));
            return;
        }
        for i in 1..=s_bytes.len() {
            let sub_s_bytes = &s_bytes[0..i];
            if Self::is_palindrome(sub_s_bytes) {
                temp.push_back(String::from_utf8_lossy(&sub_s_bytes).to_string());
                Self::backtrack(&s_bytes[i..s_bytes.len()], temp, result);
                temp.pop_back();
            }
        }
    }

    fn is_palindrome(sub_s_bytes: &[u8]) -> bool {
        let (mut left, mut right) = (0, sub_s_bytes.len() - 1);
        while left < right {
            if sub_s_bytes[left] != sub_s_bytes[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}
