use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn find_final_value(mut nums: Vec<i32>, mut original: i32) -> i32 {
        nums.sort_unstable();

        for num in nums {
            if num == original {
                original *= 2;
            }
        }
        original
    }

    pub fn find_final_value_hash_map(nums: Vec<i32>, mut original: i32) -> i32 {
        let mut store = HashSet::new();

        for num in nums {
            store.insert(num);

            while store.contains(&original) {
                original *= 2;
            }
        }
        original
    }
}
