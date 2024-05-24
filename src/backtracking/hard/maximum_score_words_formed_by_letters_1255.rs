use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        let mut score_by_word = HashMap::new();

        let mut letters_map = HashMap::new();

        for letter in &letters {
            *letters_map.entry(letter.clone()).or_insert(0) += 1;
        }
        for word in &words {
            let mut current_score = 0;
            let mut is_valid = true;
            for ch in word.chars() {
                if letters_map.contains_key(&ch) {
                    current_score += score[(ch as u8 - b'a') as usize];
                } else {
                    is_valid = false;
                    break;
                }
            }
            if is_valid {
                score_by_word.insert(word.clone(), current_score);
            } else {
                score_by_word.insert(word.clone(), -1);
            }
        }
        let mut max = 0;

        Self::backtrack(&words, &mut letters_map, &score_by_word, &mut max, 0, 0);

        max
    }

    fn backtrack(words: &Vec<String>, letters: &mut HashMap<char, i32>, score_by_word: &HashMap<String, i32>, max: &mut i32, temp: i32, start: usize) {
        if *max < temp {
            *max = temp;
        }
        for i in start..words.len() {
            if score_by_word.get(&words[i]).unwrap() != &-1 {
                Self::decrement_letters(&words[i], letters);
                if Self::is_letters_enough(letters) {
                    Self::backtrack(words, letters, score_by_word, max, temp + score_by_word.get(&words[i]).unwrap(), i + 1);
                }
                Self::increment_letters(&words[i], letters);
            }
        }
    }

    fn decrement_letters(word: &String, letters: &mut HashMap<char, i32>) {
        for ch in word.chars() {
            letters.entry(ch).and_modify(|v| *v -= 1);
        }
    }

    fn increment_letters(word: &String, letters: &mut HashMap<char, i32>) {
        for ch in word.chars() {
            letters.entry(ch).and_modify(|v| *v += 1);
        }
    }

    fn is_letters_enough(letters: &HashMap<char, i32>) -> bool {
        !letters
            .iter()
            .find(|e| e.1 < &0)
            .is_some()
    }
}
