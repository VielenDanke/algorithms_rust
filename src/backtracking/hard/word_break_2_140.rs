use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let words = HashSet::from_iter(word_dict.into_iter());

        let mut result = Vec::new();

        Self::backtrack(s.as_bytes(), &words, &mut result, &mut Vec::new());

        result
    }

    fn backtrack(s: &[u8], words: &HashSet<String>, result: &mut Vec<String>, temp: &mut Vec<String>) {
        if s.is_empty() {
            result.push(temp.clone().join(" "));
            return;
        }
        for i in 0..=s.len() {
            let to_search = String::from_utf8_lossy(&s[0..i]).to_string();
            if words.contains(&to_search) {
                temp.push(to_search);
                Self::backtrack(&s[i..s.len()], words, result, temp);
                temp.remove(temp.len() - 1);
            }
        }
    }
}
