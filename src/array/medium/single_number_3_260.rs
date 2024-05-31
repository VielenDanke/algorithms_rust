use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();

        for num in nums.iter() {
            *map.entry(*num).or_insert(0) += 1;
        }
        let mut result = Vec::with_capacity(2);

        for entry in map.iter() {
            if entry.1 == &1 {
                result.push(*entry.0);
            }
        }
        result
    }

    pub fn single_number_bit_manipulation(nums: Vec<i32>) -> Vec<i32> {
        let mut xor_all = 0;

        for &num in &nums {
            xor_all ^= num;
        }
        let set_bit = xor_all & !xor_all + 1;

        let (mut a, mut b) = (0, 0);

        for &num in &nums {
            if num & set_bit != 0 {
                a ^= num;
            } else {
                b ^= num;
            }
        }
        vec![a, b]
    }
}
