use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn maximum_happiness_sum(mut happiness: Vec<i32>, mut k: i32) -> i64 {
        happiness.sort_unstable();

        let mut decrementor = 0;

        let mut result = 0i64;

        let mut idx = (happiness.len() - 1) as i32;

        while k > 0 && idx >= 0 {
            result += (happiness[idx as usize] - decrementor).max(0) as i64;

            decrementor += 1;
            idx -= 1;
            k -= 1;
        }
        result
    }

    pub fn maximum_happiness_sum_brute_force(happiness: Vec<i32>, mut k: i32) -> i64 {
        let mut heap = BinaryHeap::new();

        happiness.iter().for_each(|&v| heap.push(v));

        let mut decrementor = 0;

        let mut result = 0i64;

        while k > 0 && !heap.is_empty(){
            let last_val = heap.pop().unwrap();

            result += (last_val as i64 - decrementor).max(0);

            decrementor += 1;

            k -= 1;
        }

        result
    }
}