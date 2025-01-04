use std::collections::HashSet;

pub struct Solution;

impl Solution {
    fn count_palindromic_subsequence(s: String) -> i32 {
        let mut letters: HashSet<u8> = HashSet::new();
        let s_bytes = s.as_bytes();

        for &c in s_bytes.iter() {
            letters.insert(c);
        }
        let mut result = 0;

        for letter in letters {
            let (mut i, mut j) = (-1, 0);

            for (k, &c) in s_bytes.iter().enumerate() {
                if c == letter {
                    if i == -1 {
                        i = k as i32;
                    }
                    j = k as i32;
                }
            }
            let mut between: HashSet<u8> = HashSet::new();

            for k in (i + 1)..j {
                between.insert(s_bytes[k as usize]);
            }
            result += between.len() as i32;
        }
        result
    }

    fn count_palindromic_subsequence_alph(s: String) -> i32 {
        let mut first = vec![None; 26];
        let mut last = vec![0; 26];
        let s_bytes = s.as_bytes();
        let mut result = 0;
        let n = s_bytes.len();

        for i in 0..n {
            let current = s_bytes[i];
            let idx = (current - b'a') as usize;

            if first[idx].is_none() {
                first[idx] = Some(i);
            }
            last[idx] = i;
        }
        for i in 0..26 {
            if first[i].is_none() {
                continue;
            }
            let mut visited = HashSet::new();

            for j in first[i].unwrap() + 1..last[i] {
                visited.insert(s_bytes[j]);
            }
            result += visited.len() as i32;
        }
        result
    }
}
