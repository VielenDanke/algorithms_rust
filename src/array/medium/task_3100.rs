pub struct Solution;

impl Solution {
    pub fn max_bottles_drunk(num_bottles: i32, mut num_exchange: i32) -> i32 {
        let mut x = num_bottles;
        let mut y = 0;
        while x >= num_exchange {
            x -= num_exchange;
            y += 1;
            num_exchange += 1;
            x += 1;
        }
        num_bottles + y
    }
}