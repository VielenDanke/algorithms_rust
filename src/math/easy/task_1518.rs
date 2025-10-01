pub struct Solution;

impl Solution {
    pub fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
        // num_exchange bottles = 1 num_bottles
        let mut result = num_bottles;

        while num_bottles > 1 && num_bottles >= num_exchange {
            let current = num_bottles / num_exchange;
            result += current;
            num_bottles = current + (num_bottles % num_exchange);
        }
        result
    }

    pub fn num_water_bottles_one_line(mut num_bottles: i32, num_exchange: i32) -> i32 {
        num_bottles + (num_bottles - 1) / (num_exchange - 1)
    }
}
