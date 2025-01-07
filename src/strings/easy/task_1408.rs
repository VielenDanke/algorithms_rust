pub struct Solution;

impl Solution {
    // count on possibility to match with the biggest word
    pub fn string_matching_with_sorting(mut words: Vec<String>) -> Vec<String> {
        words.sort_unstable_by(|left, right| left.len().cmp(&right.len()));

        let mut result = Vec::new();

        for (i, word) in words.iter().enumerate() {
            for j in ((i + 1)..words.len()).rev() {
                if words[j].contains(word) {
                    result.push(word.clone());
                    break;
                }
            }
        }
        result
    }

    pub fn string_matching_with(words: Vec<String>) -> Vec<String> {
        words
            .iter()
            .filter_map(|word| {
                let does_fit = words
                    .iter()
                    .any(|other_word| other_word.contains(word) && !other_word.eq(word));
                if does_fit {
                    Some(word.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}
