use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn count_of_substrings_faster(word: String, k: i32) -> i64 {
        let mut num_valid_substrings = 0;
        let mut start = 0;
        let mut end = 0;
        let k = k as usize;
        let mut vowel_count: HashMap<char, usize> = HashMap::new();
        let mut consonant_count = 0;
        let chars: Vec<char> = word.chars().collect();

        // Compute index of next consonant for all indices
        let mut next_consonant = vec![word.len(); word.len()];
        let mut next_consonant_index = word.len();
        for i in (0..word.len()).rev() {
            next_consonant[i] = next_consonant_index;
            if !Self::is_vowel(chars[i]) {
                next_consonant_index = i;
            }
        }

        // Start sliding window
        while end < word.len() {
            let new_letter = chars[end];

            // Update counts
            if Self::is_vowel(new_letter) {
                *vowel_count.entry(new_letter).or_insert(0) += 1;
            } else {
                consonant_count += 1;
            }

            // Shrink window if too many consonants in the window
            while consonant_count > k {
                let start_letter = chars[start];
                if Self::is_vowel(start_letter) {
                    if let Some(count) = vowel_count.get_mut(&start_letter) {
                        *count -= 1;
                        if *count == 0 {
                            vowel_count.remove(&start_letter);
                        }
                    }
                } else {
                    consonant_count -= 1;
                }
                start += 1;
            }

            // While we have a valid window, try to shrink it
            while start < word.len()
                && vowel_count.len() == 5
                && consonant_count == k
            {
                num_valid_substrings += (next_consonant[end] - end) as i64;
                let start_letter = chars[start];
                if Self::is_vowel(start_letter) {
                    if let Some(count) = vowel_count.get_mut(&start_letter) {
                        *count -= 1;
                        if *count == 0 {
                            vowel_count.remove(&start_letter);
                        }
                    }
                } else {
                    consonant_count -= 1;
                }
                start += 1;
            }
            end += 1;
        }

        num_valid_substrings
    }

    fn is_vowel(c: char) -> bool {
        matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
    }
    
    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        fn eq_vowels(substring: &[u8], mut k: i32) -> bool {
            let mut alph = HashMap::new();

            alph.insert((b'a' - b'a') as usize, false);
            alph.insert((b'e' - b'a') as usize, false);
            alph.insert((b'i' - b'a') as usize, false);
            alph.insert((b'o' - b'a') as usize, false);
            alph.insert((b'u' - b'a') as usize, false);
            
            for &letter in substring.iter() {
                if !alph.contains_key(&((letter - b'a') as usize)) {
                    k -= 1;
                } else {
                    alph.entry((letter - b'a') as usize)
                        .and_modify(|v| *v = true);
                }
            }
            if k != 0 {
                return false;
            }
            alph.values().all(|v| *v == true)
        }

        let mut window = (5 + k) as usize;

        let word_bytes = word.as_bytes();
        let n = word_bytes.len();

        let mut counter = 0;

        while window <= n {
            let mut idx = 0;

            while idx + window <= n {
                let substring = &word_bytes[idx..idx + window];

                if eq_vowels(&substring, k) {
                    counter += 1;
                }
                idx += 1;
            }
            window += 1;
        }
        counter
    }
}
