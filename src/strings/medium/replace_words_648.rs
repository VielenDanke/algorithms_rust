pub struct Solution;

impl Solution {
    pub fn replace_words(mut dictionary: Vec<String>, sentence: String) -> String {
        dictionary.sort_unstable();

        sentence
            .split(" ")
            .map(|s| Self::replace_with_word_if_needed(String::from(s), &dictionary))
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn replace_with_word_if_needed(word: String, dictionary: &Vec<String>) -> String {
        for dictionary_word in dictionary.iter() {
            if word.starts_with(dictionary_word) {
                return dictionary_word.clone();
            }
        }
        word
    }
}
