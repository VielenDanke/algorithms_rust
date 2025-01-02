use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut prefix_sum = vec![0; words.len()];
        let mut vowels: HashSet<u8> = HashSet::from_iter(vec![b'a', b'e', b'i', b'o', b'u']);
        let first_word_bytes = words[0].as_bytes();

        prefix_sum[0] = if vowels.contains(&first_word_bytes[0])
            && vowels.contains(&first_word_bytes[first_word_bytes.len() - 1])
        {
            1
        } else {
            0
        };

        for i in 1..words.len() {
            let word = words[i].as_bytes();

            prefix_sum[i] = prefix_sum[i - 1]
                + if vowels.contains(&word[0]) && vowels.contains(&word[word.len() - 1]) {
                    1
                } else {
                    0
                };
        }
        queries
            .into_iter()
            .enumerate()
            .map(|(i, q)| {
                prefix_sum[q[1] as usize]
                    - if q[0] == 0 {
                        0
                    } else {
                        prefix_sum[(q[0] - 1) as usize]
                    }
            })
            .collect::<Vec<i32>>()
    }
}
