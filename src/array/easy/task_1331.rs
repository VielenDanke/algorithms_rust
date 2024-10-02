use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn array_rank_transform(mut arr: Vec<i32>) -> Vec<i32> {
        let mut c_arr = arr.clone();

        c_arr.sort_unstable();

        let mut map = HashMap::new();

        let mut rank = 1;

        for num in c_arr.iter() {
            if map.contains_key(num) {
                continue;
            }
            map.insert(*num, rank);
            rank += 1;
        }
        arr.iter().map(|num| *map.get(num).unwrap()).collect()
    }
}