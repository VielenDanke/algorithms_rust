pub struct Solution;

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let mut ones = 0;
        let mut zeroes = 0;
        let mut result = 0;

        for ch in s.chars() {
            if ch == '1' {
                ones += 1;
            }
        }
        for ch in s.chars().take(s.len() - 1) {
            if ch == '0' {
                zeroes += 1;
            } else {
                ones -= 1;
            }
            result = result.max(zeroes + ones);
        }
        result
    }
}