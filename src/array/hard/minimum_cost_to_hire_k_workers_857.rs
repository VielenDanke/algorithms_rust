use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;

        let mut workers = quality.iter().enumerate().map(|(i, &q)| (wage[i] as f64 / q as f64, q)).collect::<Vec<(f64, i32)>>();

        workers.sort_unstable_by(|left, right| left.0.partial_cmp(&right.0).unwrap());

        let mut qsum = 0i32;

        let mut h = BinaryHeap::<i32>::new();

        workers.iter().fold(f64::MAX, |res, &(r, q)| {
            h.push(q);
            qsum += q;
            if h.len() > k {
                qsum -= h.pop().unwrap();
            }
            if h.len() == k {
                res.min(qsum as f64 * r)
            } else {
                res
            }
        })
    }
}
