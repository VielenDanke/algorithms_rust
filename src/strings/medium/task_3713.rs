pub struct Solution;

impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        let s_bytes = s.as_bytes();
        let n = s_bytes.len();
        let mut result = 0;

        for i in 0..n {
            let mut alph = vec![0; 26];

            for j in i..n {
                let idx = (s_bytes[j] - b'a') as usize;

                alph[idx] += 1;

                if alph.iter().filter(|&&k| k != 0).all(|&k| k == alph[idx]) {
                    result = result.max(j - i + 1);
                }
            }
        }

        result as i32
    }

    pub fn longest_balanced_faster(s: String) -> i32 {
        let s_bytes = s.as_bytes();
        let n = s_bytes.len();
        let mut max_len = 0;

        // Outer loop: Starting point of the substring
        for l in 0..n {
            let mut freq = [0; 26];
            let mut unique_chars = 0;
            let mut max_f = 0;
            let mut count_with_max_f = 0;

            // Inner loop: Ending point of the substring
            for r in l..n {
                let idx = (s_bytes[r] - b'a') as usize;

                // If this is a new character, increment unique count
                if freq[idx] == 0 {
                    unique_chars += 1;
                }

                freq[idx] += 1;
                let f = freq[idx];

                if f > max_f {
                    max_f = f;
                    count_with_max_f = 1; // New max frequency found, reset count to 1
                } else if f == max_f {
                    count_with_max_f += 1; // Another character reached the max frequency
                }

                // A substring is balanced ONLY if every unique character
                // present in the window appears exactly max_f times.
                if unique_chars == count_with_max_f {
                    max_len = max_len.max((r - l + 1) as i32);
                }
            }
        }
        max_len
    }
}
