pub struct Solution;

impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let mut char_1: u8 = 0;
        let mut char_2: u8 = 0;

        let split_sentence = sentence.split(" ")
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>();

        for (i, word) in split_sentence.iter().enumerate() {
            let current_word_bytes = word.as_bytes();
            if i == 0 {
                char_1 = current_word_bytes[0];
            } else {
                let prev_word_bytes = split_sentence[i - 1].as_bytes();
                if prev_word_bytes[prev_word_bytes.len() - 1] != current_word_bytes[0] {
                    return false;
                }
            }
            char_2 = current_word_bytes[current_word_bytes.len() - 1];
        }
        char_1 == char_2
    }

    pub fn is_circular_sentence_better(sentence: String) -> bool {
        let sentence_bytes = sentence.as_bytes();
        let char_1 = sentence_bytes[0];
        let char_2 = sentence_bytes[sentence_bytes.len() - 1];

        if char_1 != char_2 {
            false
        } else {
            for i in 1..sentence_bytes.len()-1 {
                if sentence_bytes[i] == b' ' && sentence_bytes[i - 1] != sentence_bytes[i + 1] {
                    return false;
                }
            }
            true
        }
    }
}