pub struct Solution;

impl Solution {
    pub fn successful_pairs_fast(mut spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        let n = potions.len();

        potions.sort_unstable();

        for spell in spells.iter_mut() {
            let left = Self::binary_search(&potions, success, *spell as i64);
            *spell = (n - left) as i32;
        }
        spells
    }

    fn binary_search(arr: &Vec<i32>, to_add: i64, current_spell: i64) -> usize {
        let mut left = 0;
        let mut right = arr.len();

        while left < right {
            let middle = left + (right - left) / 2;

            if (arr[middle] as i64 * current_spell) >= to_add {
                right = middle;
            } else {
                left = middle + 1;
            }
        }
        left
    }

    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut result = vec![0; spells.len()];

        for (i, &spell) in spells.iter().enumerate() {
            for &potion in potions.iter() {
                if potion as i64 * spell as i64 >= success {
                    result[i] += 1;
                }
            }
        }
        result
    }
}
