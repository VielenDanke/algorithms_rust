use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn special_triplets(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let (mut left_map, mut right_map) = (HashMap::new(), HashMap::new());

        nums.iter().for_each(|&v| *right_map.entry(v as i64).or_insert(0) += 1);

        let mut total_comb = 0;

        nums.iter()
            .for_each(|&v| {
                *right_map.entry(v as i64).or_insert(0i64) -= 1;

                let target = v as i64 * 2;

                let left = *left_map.entry(target).or_default();
                let right = *right_map.entry(target).or_default();

                if left > 0 && right > 0 {
                    let current_comb = (left * right) % MOD;
                    total_comb = (total_comb + current_comb) % MOD;
                }

                *left_map.entry(v as i64).or_insert(0i64) += 1;
            });

        total_comb as i32
    }
}