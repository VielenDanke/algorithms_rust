use std::collections::HashMap;

pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
    let mut seqs = Vec::<HashMap<i32, i32>>::new();
    let mut result = 0;
    for (i, x) in nums.iter().copied().enumerate() {
        let mut new_seqs = HashMap::new();
        for (j, y) in nums[..i].iter().copied().enumerate() {
            let diff = match x.checked_sub(y) {
                None => continue,
                Some(diff) => diff,
            };
            let count = seqs[j].get(&diff).copied().unwrap_or(0);
            result += count;
            if x.checked_add(diff).is_some() {
                *(new_seqs.entry(diff).or_insert(0)) += count + 1;
            }
        }
        seqs.push(new_seqs);
    }
    result
}
