use std::collections::BTreeMap;

pub struct Solution;

impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut cloned_array = heights.clone();

        cloned_array.sort_unstable();

        let mut counter = 0;

        for i in 0..heights.len() {
            if cloned_array[i] != heights[i] {
                counter += 1;
            }
        }
        counter
    }

    pub fn height_checker_b_tree(heights: Vec<i32>) -> i32 {
        let mut b_tree_map = BTreeMap::new();

        for height in heights {
            *b_tree_map.entry(height).or_insert(0) += 1;
        }

        let mut idx = 0;
        let mut counter = 0;

        while idx < heights.len() {
            if let Some((key, mut value)) = b_tree_map.pop_first() {
                while value > 0 {
                    if key != heights[idx] {
                        counter += 1;
                    }
                    value -= 1;
                    idx += 1;
                }
            }
        }

        counter
    }
}
