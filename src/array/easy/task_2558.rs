use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, mut k: i32) -> i64 {
        let mut heap: BinaryHeap<i64> = BinaryHeap::new();

        gifts.iter().for_each(|&gift| heap.push(gift as i64));

        while k > 0 {
            if let Some(elem) = heap.pop() {
                let new_elem = Self::floor_sqrt(elem);
                heap.push(new_elem);
            }
            k -= 1;
        }
        heap.iter().map(|&gift| gift).sum()
    }

    fn floor_sqrt(n: i64) -> i64 {
        if n <= 1 {
            return n;
        }

        let mut low = 1;
        let mut high = n;

        while low <= high {
            let mid = (low + high) / 2;
            let square = mid * mid;

            if square == n {
                return mid;
            } else if square < n {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }

        high // Return the floor
    }
}