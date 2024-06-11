use std::collections::BTreeMap;

pub struct Solution;

impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut ordered_map = BTreeMap::new();

        for &num in arr1.iter() {
            *ordered_map.entry(num).or_insert(0) += 1;
        }

        let mut result = Vec::new();

        for num in arr2.iter() {
            let mut value = ordered_map.remove(num).unwrap();
            while value > 0 {
                result.push(*num);
                value -= 1;
            }
        }

        while let Some((k, mut v)) = ordered_map.pop_first() {
            while v > 0 {
                result.push(k);
                v -= 1;
            }
        }

        result
    }
}
