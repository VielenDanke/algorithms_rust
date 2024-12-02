pub struct Solution;

impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        sentence
            .split(" ")
            .into_iter()
            .enumerate()
            .find_map(|(i, word)| {
                if word.starts_with(&search_word) {
                    Some((i as i32) + 1)
                } else {
                    None
                }
            })
            .unwrap_or(-1)
    }
}