pub struct Solution;

impl Solution {
    pub fn replace_words(mut dictionary: Vec<String>, sentence: String) -> String {
        dictionary.sort_unstable();

        sentence
            .split(" ")
            .map(|s| String::from(s))
            .map(|s| {
                for dictionary_word in dictionary.iter() {
                    if s.starts_with(dictionary_word) {
                        return dictionary_word.clone();
                    }
                }
                s
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}
