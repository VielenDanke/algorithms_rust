struct Solution;

impl Solution {
    pub fn wonderful_substrings(word: String) -> i64 {
        let mut ans = 0i64;
        let mut count = [0i64; 1024];
        count[0] = 1;
        let mut acc = 0usize;
        for b in word.bytes() {
            let bit = 1usize << (b - b'a');
            acc ^= bit;
            ans += count[acc];
            for i in 0..10 {
                ans += count[acc ^ (1usize << i)];
            }
            count[acc] += 1;
        }
        ans
    }

    pub fn wonderful_substrings_sliding_window(word: String) -> i64 {
        let word_bytes = word.as_bytes();

        let (mut window, mut result) = (1, 0);

        while window <= word.len() {
            let mut i = 0;

            while i + window <= word.len() {
                let slice = &word_bytes[i..i+window];

                let mut counter = vec![0; 26];

                for v in slice {
                    counter[(*v-'a' as u8) as usize] += 1;
                }
                let mut count_odd = 0;

                for v in counter {
                    if v % 2 != 0 {
                        count_odd += 1;
                    }
                }
                if count_odd <= 1 {
                    result += 1;
                }
                i += 1;
            }
            window += 1;
        }
        result
    }
}
