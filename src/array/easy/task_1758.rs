pub struct Solution;

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut swaps = 0;
        let mut current = '0';

        for b in s.chars() {
            if b != current {
                swaps += 1;
            }
            if current == '0' {
                current = '1';
            } else {
                current = '0';
            }
        }

        swaps.min(s.len() as i32 - swaps)
    }
}