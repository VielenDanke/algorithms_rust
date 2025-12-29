pub struct Solution;

impl Solution {
    pub fn best_closing_time(customers: String) -> i32 {
        let (mut min_penalty, mut current_penalty, mut min_hour) = (0, 0, 0);

        for (i, customer) in customers.chars().enumerate() {
            if customer == 'Y' {
                current_penalty -= 1;
            } else {
                current_penalty += 1;
            }
            if min_penalty > current_penalty {
                min_penalty = current_penalty;
                min_hour = i + 1;
            }
        }
        min_hour as i32
    }
}