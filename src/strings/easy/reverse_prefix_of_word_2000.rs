struct Solution;

impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let word_bytes = word.as_bytes();

        if let Some(idx) = word.find(ch) {
            let (mut left, mut right) = word_bytes.split_at(idx + 1);
            let mut left_vec = left.to_vec();
            left_vec.reverse();
            left_vec.append(&mut right.to_vec());
            return String::from_utf8(left_vec).unwrap();
        }
        word
    }
}
