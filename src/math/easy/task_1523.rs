pub struct Solution;

impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        (high - low) / 2
            + if low % 2 != 0 {
                1
            } else if high % 2 != 0 {
                1
            } else {
                0
            }
    }

    pub fn count_odds_brute_force(low: i32, high: i32) -> i32 {
        (low..=high).filter(|&num| num % 2 != 0).count() as i32
    }
}
