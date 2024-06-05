pub struct Solution;

const ARRAY_LEN: usize = 26usize;

impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {

        let mut common_chars = [0usize; ARRAY_LEN];

        for c in words[0].chars() {
            common_chars[(c as u8 - b'a') as usize] += 1;
        }

        for word in words.iter().skip(1) {
            let mut word_chars = [0usize; ARRAY_LEN];
            for c in word.chars() {
                word_chars[(c as u8 - b'a') as usize] += 1;
            }
            for i in 0..ARRAY_LEN {
                common_chars[i] = common_chars[i].min(word_chars[i]);
            }
        }

        let mut result = Vec::new();

        for i in 0..ARRAY_LEN {
            for _ in 0..common_chars[i] {
                result.push(((b'a' + i as u8) as char).to_string());
            }
        }
        result
    }
}
