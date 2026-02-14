use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        // control distinct odd and even numbers on all the steps
        let n = nums.len();

        let mut result = 0;

        for i in 0..n {
            let mut even_map: HashMap<i32, i32> = HashMap::new();
            let mut odd_map: HashMap<i32, i32> = HashMap::new();

            for j in i..n {
                let m = if nums[j] % 2 == 0 { &mut even_map } else { &mut odd_map };
                *m.entry(nums[j]).or_insert(0) += 1;
                if even_map.len() == odd_map.len() {
                    result = result.max(j - i + 1);
                }
            }
        }

        result as i32
    }
}
