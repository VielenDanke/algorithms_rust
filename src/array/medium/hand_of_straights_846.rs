use std::collections::{HashMap, BinaryHeap};
use std::collections::hash_map::Entry;

pub struct Solution;

impl Solution {
    pub fn is_n_straight_hand(mut hand: Vec<i32>, group_size: i32) -> bool {
        let n = hand.len();

        if n % (group_size as usize) != 0 {
            return false;
        }
        hand.sort_unstable();

        let number_of_arrays = n / group_size as usize;

        let mut expected_arrays: Vec<Vec<i32>> = vec![vec![]; number_of_arrays];

        let mut current_idx = 0;

        for num in hand.iter() {
            for idx in current_idx..expected_arrays.len() {
                let next_array = expected_arrays.get_mut(idx).unwrap();
                if next_array.is_empty() || (next_array[next_array.len() - 1].abs_diff(*num) == 1 && next_array.len() < group_size as usize) {
                    next_array.push(*num);
                    if next_array.len() == group_size as usize {
                        current_idx = idx;
                    }
                    break;
                }
            }
        }
        for arr in expected_arrays {
            if arr.len() != group_size as usize {
                return false;
            }
        }
        true
    }

    pub fn is_n_straight_hand_faster(hand: Vec<i32>, group_size: i32) -> bool {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut heap: BinaryHeap<i32> = BinaryHeap::new();
        let mut smallest: i32;
        let mut occurance: i32;

        for card in hand {
            heap.push(-card);
            *map.entry(card).or_insert(0) += 1;
        }

        while heap.len() > 0 {
            smallest = -heap.pop().unwrap();
            occurance = *map.get(&smallest).unwrap();
            if occurance > 0 {
                for i in 0..group_size {
                    match map.entry(smallest + i) {
                        Entry::Vacant(_) => {
                            return false;
                        }
                        Entry::Occupied(mut x) => {
                            *x.get_mut() -= occurance;
                        }
                    }
                }
            }
            if occurance < 0 {
                return false;
            }
        }
        true
    }
}
