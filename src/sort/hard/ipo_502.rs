use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn find_maximized_capital(k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let n = profits.len();

        let mut capital_positions = (0..n).collect::<Vec<usize>>();

        capital_positions.sort_unstable_by(|&a, &b| capital[a].cmp(&capital[b]));

        let mut heap = BinaryHeap::with_capacity(n);

        let mut i = 0;

        for _ in 0..k {
            // add profits while we are able to with capital `w`
            while i < n && capital[capital_positions[i]] <= w {
                heap.push(profits[capital_positions[i]]);
                i += 1;
            }

            // pop the biggest profit and add it to current capital `w`
            // in case we don't have anything in the heap meaning we aren't able to accept more
            // return current capital
            match heap.pop() {
                Some(v) => w += v,
                None => return w,
            }
        }

        w
    }
}
