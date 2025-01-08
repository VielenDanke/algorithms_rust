pub struct Solution;

impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        let n = words.len();
        let mut counter = 0;

        for (i, word) in words.iter().enumerate() {
            for j in (i + 1)..n {
                let to_check = words.get(j).unwrap();
                if to_check.starts_with(word) && to_check.ends_with(word) {
                    counter += 1;
                }
            }
        }
        counter
    }
}