use std::collections::{VecDeque, HashSet};

pub struct Solution {}

/// Queue, String, Array
impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let mut ignore: HashSet<_> = deadends.iter().map(|s| s.parse().unwrap()).collect();
        let (target, mut queue, mut moves) = (target.parse::<u16>().unwrap(), VecDeque::new(), 0);
        let array = [1000, 0100, 0010, 0001];
        queue.push_back(0000);
        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let lock = queue.pop_front().unwrap();
                if lock == target {
                    return moves;
                } else if ignore.insert(lock) {
                    for i in &array {
                        let wheel = (lock / i) % 10;
                        queue.push_back((lock - i * wheel) + (i * ((wheel + 1) % 10))); // +1
                        queue.push_back((lock - i * wheel) + (i * ((wheel + 9) % 10))); // -1
                    }
                }
            }
            moves += 1;
        }
        -1
    }
}
