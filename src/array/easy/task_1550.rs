pub struct Solution;

impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        arr.windows(3).any(|w| w[0] % 2 != 0 && w[1] % 2 != 0 && w[2] % 2 != 0)
    }
}