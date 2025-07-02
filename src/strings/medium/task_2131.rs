use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn longest_palindrome_faster(words: Vec<String>) -> i32 {
        let mut open = [0; 26 * 26];
        let mut total = 0;
        for word in words {
            let a = word.as_bytes()[0] as usize - 'a' as usize;
            let b = word.as_bytes()[1] as usize - 'a' as usize;
            let hash = a * 26 + b;
            let inv_hash = b * 26 + a;

            if open[inv_hash] > 0 {
                open[inv_hash] -= 1;
                total += 4;
            } else {
                open[hash] += 1;
            }
        }
        for i in 0..26 {
            if open[i * 26 + i] > 0 {
                total += 2;
                break;
            }
        }

        total
    }
    
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut storage: HashMap<String, i32> = HashMap::new();
        for word in &words {
            *storage.entry(word.clone()).or_insert(0) += 1;
        }

        let mut result = 0;
        let mut is_found = false;

        for word in &words {
            let chars: Vec<char> = word.chars().collect();
            if chars[0] == chars[1] {
                let count = *storage.get(word).unwrap_or(&0);
                if count > 0 {
                    let mut word_amount = count;
                    if word_amount % 2 != 0 {
                        word_amount -= 1;
                        is_found = true;
                    }
                    result += word_amount * 2;
                    storage.insert(word.clone(), count - word_amount);
                }
            } else {
                let rev = format!("{}{}", chars[1], chars[0]);
                let count1 = *storage.get(word).unwrap_or(&0);
                let count2 = *storage.get(&rev).unwrap_or(&0);
                if count1 > 0 && count2 > 0 {
                    let min = count1.min(count2);
                    result += min * 4;
                    storage.insert(word.clone(), count1 - min);
                    storage.insert(rev, count2 - min);
                }
            }
        }

        if is_found {
            result += 2;
        }

        result
    }
}
