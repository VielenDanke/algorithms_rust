/*
You are given a string s and two integers x and y. You can perform two types of operations any number of times.

Remove substring "ab" and gain x points.
For example, when removing "ab" from "cabxbae" it becomes "cxbae".
Remove substring "ba" and gain y points.
For example, when removing "ba" from "cabxbae" it becomes "cabxe".
Return the maximum points you can gain after applying the above operations on s.
 */

pub struct Solution;

impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        let (high_pair, high_score, low_pair, low_score) = if x > y {
            (('a', 'b'), x, ('b', 'a'), y)
        } else {
            (('b', 'a'), y, ('a', 'b'), x)
        };

        let (s_after_high, high_count) = Self::remove_pair(&s, high_pair.0, high_pair.1);
        let (_, low_count) = Self::remove_pair(&s_after_high, low_pair.0, low_pair.1);

        high_count * high_score + low_count * low_score
    }

    fn remove_pair(s: &str, first: char, second: char) -> (String, i32) {
        let mut stack: Vec<char> = Vec::new();
        let mut count = 0;

        for c in s.chars() {
            if c == second {
                if let Some(&last) = stack.last() {
                    if last == first {
                        stack.pop();
                        count += 1;
                        continue;
                    }
                }
            }
            stack.push(c);
        }
        let remaining: String = stack.into_iter().collect();

        (remaining, count)
    }
}
