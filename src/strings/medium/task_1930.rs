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

    pub fn count_palindromic_subsequence_shorter(s: String) -> i32 {
        let s_bytes = s.as_bytes();
        let mut total_count = 0;

        for byte in b'a'..=b'z' {
            let first_idx = s_bytes.iter().position(|&x| x == byte);

            let last_idx = s_bytes.iter().rposition(|&x| x == byte);

            match (first_idx, last_idx) {
                (Some(f), Some(l)) if l > f + 1 => {
                    let mut unique_middles = HashSet::new();

                    for &middle_char in &s_bytes[f + 1..l] {
                        unique_middles.insert(middle_char);
                    }

                    total_count += unique_middles.len();
                }
                _ => {}
            }
        }

        total_count as i32
    }

    pub fn count_palindromic_subsequence_bits(s: String) -> i32 {
        let s_bytes = s.as_bytes();

        // Arrays to store the first and last occurrence of each character (0-25)
        // We initialize 'first' with usize::MAX to represent "not found yet"
        let mut first = [usize::MAX; 26];
        let mut last = [0; 26];

        // 1. Single pass to record positions (O(N))
        for (i, &byte) in s_bytes.iter().enumerate() {
            let idx = (byte - b'a') as usize;
            if first[idx] == usize::MAX {
                first[idx] = i;
            }
            last[idx] = i;
        }

        let mut total_count = 0;

        // 2. Check each character 'a' through 'z' (Constant time: O(26))
        for i in 0..26 {
            // Check if the character exists AND if there is space between first and last
            if first[i] != usize::MAX && last[i] > first[i] + 1 {
                // We need to count unique chars in the range (first[i] + 1 .. last[i])
                // We use a u32 bitmask as a "lightweight HashSet"
                let mut bitmask: u32 = 0;

                for &middle_char in &s_bytes[first[i] + 1..last[i]] {
                    let char_bit = (middle_char - b'a') as u32;
                    // The '|=' operator sets the bit corresponding to that char to 1
                    bitmask |= 1 << char_bit;
                }

                // .count_ones() is a highly optimized CPU instruction
                total_count += bitmask.count_ones();
            }
        }

        total_count as i32
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

    pub fn count_palindromic_subsequence_backtrack(s: String) -> i32 {
        let mut visited = HashSet::new();

        let s_bytes = s.as_bytes();

        fn backtrack(
            s_bytes: &[u8],
            temp: &mut Vec<u8>,
            idx: usize,
            visited: &mut HashSet<String>,
        ) {
            if temp.len() == 3 {
                if temp[0] == temp[2] {
                    visited.insert(String::from_utf8(temp.clone()).unwrap());
                }
                return;
            }
            for i in idx..s_bytes.len() {
                temp.push(s_bytes[i]);
                backtrack(s_bytes, temp, i + 1, visited);
                temp.pop();
            }
        }
        backtrack(s_bytes, &mut vec![], 0, &mut visited);

        visited.len() as i32
    }
}
