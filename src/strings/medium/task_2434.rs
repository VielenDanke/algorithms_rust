use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn robot_with_string(s: String) -> String {
        fn smallest_char(freq: &[i32; 26]) -> char {
            for (i, &count) in freq.iter().enumerate() {
                if count > 0 {
                    return (b'a' + i as u8) as char;
                }
            }
            'a'
        }
        
        let mut stack: VecDeque<char> = VecDeque::new();
        let mut freq = [0; 26];

        // Count frequency of each character
        for ch in s.chars() {
            freq[(ch as u8 - b'a') as usize] += 1;
        }

        let mut t = String::new();

        for ch in s.chars() {
            stack.push_back(ch);
            freq[(ch as u8 - b'a') as usize] -= 1;

            while let Some(&top) = stack.back() {
                if top <= smallest_char(&freq) {
                    t.push(stack.pop_back().unwrap());
                } else {
                    break;
                }
            }
        }

        // Append remaining characters from stack
        while let Some(ch) = stack.pop_back() {
            t.push(ch);
        }

        t
    }
}