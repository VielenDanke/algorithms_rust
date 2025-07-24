pub struct Solution;

impl Solution {
    pub fn is_valid(word: String) -> bool {
        let word_bytes = word.as_bytes().to_ascii_lowercase();

        let mut vowels = 0;
        let mut consonant = 0;

        for &b in word_bytes.iter() {
            let is_ascii_alphabetic = b.is_ascii_alphabetic();
            
            if is_ascii_alphabetic || b.is_ascii_digit() {
                if b == b'a' || b == b'e' || b == b'i' || b == b'o' || b == b'u' {
                    vowels += 1;
                } else if is_ascii_alphabetic {
                    consonant += 1;
                }
                continue;
            }
            return false;
        }
        vowels >= 1 && consonant >= 1 && word_bytes.len() >= 3
    }
}   