use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        fn solve(s: &[u8], p: [u8; 2], score: i32) -> (i32, VecDeque<u8>) {
            let mut stack = VecDeque::new();
            let mut result = 0;

            for &ch in s.iter() {
                if ch == p[1] && stack.back() == Some(&p[0]) {
                    stack.pop_back();
                    result += score;
                } else {
                    stack.push_back(ch);
                }
            }
            (result, stack)
        }

        if x > y {
            let (result, stack) = solve(s.as_bytes(), [b'a', b'b'], x);
            result + solve(stack.as_slices().0, [b'b', b'a'], y).0
        } else {
            let (result, stack) = solve(s.as_bytes(), [b'b', b'a'], y);
            result + solve(stack.as_slices().0, [b'a', b'b'], x).0
        }
    }

    pub fn maximum_gain_backtrack(s: String, x: i32, y: i32) -> i32 {
        let to_process = s.chars()
            .map(|ch| ch as u8)
            .collect::<Vec<u8>>();

        Self::backtrack(to_process, x, y, 0)
    }

    fn backtrack(s: Vec<u8>, x: i32, y: i32, score: i32) -> i32 {
        if s.is_empty() {
            return score;
        }
        let mut max_score = 0;

        for i in 0..s.len() - 1 {
            if s[i] == b'a' && s[i + 1] == b'b' {
                max_score = max_score.max(Self::backtrack([&s[..i], &s[i + 2..]].concat(), x, y, score + x));
            }

            if s[i] == b'b' && s[i + 1] == b'a' {
                max_score = max_score.max(Self::backtrack([&s[..i], &s[i + 2..]].concat(), x, y, score + y));
            }
        }

        return max_score.max(score);
    }
}
