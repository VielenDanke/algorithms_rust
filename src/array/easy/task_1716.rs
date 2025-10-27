pub struct Solution;

impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let mut incr = 0;
        let mut result = 0;

        for i in 0..n {
            let current_week_day = i % 7;

            if current_week_day == 0 {
                incr += 1;
            }
            result += (current_week_day) + incr;
        }
        result
    }
}