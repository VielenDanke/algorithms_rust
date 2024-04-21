struct Solution {}

impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;

        let mut tickets = Vec::from(tickets);

        while tickets[k as usize] > 0 {
            for i in 0..tickets.len() {
                if tickets[i] != 0 {
                    tickets[i] -= 1;
                    result += 1;
                }
                if tickets[k as usize] == 0 {
                    return result;
                }
            }
        }
        result
    }

    pub fn time_required_to_buy_without_while_cycle(tickets: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;

        for (i, v) in tickets.iter().enumerate() {
            let current_min = *v.min(&tickets[k as usize]);
            if i <= k as usize || &tickets[k as usize] > v {
                result += current_min;
            } else {
                result += current_min - 1;
            }
        }
        result
    }
}
