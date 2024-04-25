pub struct Solution {}

impl Solution {
    pub fn longest_ideal_string(s: String, k: i32) -> i32 {
        let (k, mut dp) = (k as usize, vec![0; 256]);
        for b in s.bytes() {
            let b = b as usize;
            // 1 + (check every letter in the range b - k and b + k, find max using dp[i], where i - current letter)
            dp[b] = 1 + ((b - k)..=(b + k)).fold(0, |val, i| val.max(dp[i]));
        }
        *dp.iter().max().unwrap()
    }

    pub fn longest_ideal_string_tle(s: String, k: i32) -> i32 {
        let n = s.len();
        let mut count_ideal = vec![1; n];
        let mut result = -1 << 30;

        let s_bytes = s.as_bytes();

        for i in 0..n {
            for j in 0..i {
                if s_bytes[i].abs_diff(s_bytes[j]) <= k as u8 {
                    count_ideal[i] = count_ideal[i].max(count_ideal[j] + 1);
                    result = result.max(count_ideal[i]);
                }
            }
        }
        result
    }

    pub fn longest_ideal_string_recursive_tle(s: String, k: i32) -> i32 {
        fn dfs(s_bytes: &[u8], cache: &mut Vec<Option<i32>>, k: u8, idx: usize) -> i32 {
            if idx >= s_bytes.len() {
                return 0;
            }
            if cache[idx].is_some() {
                return cache[idx].unwrap();
            }
            let mut current_max = 0;
            for i in idx..s_bytes.len() {
                if s_bytes[i].abs_diff(s_bytes[idx]) <= k && i > idx {
                    current_max = current_max.max(dfs(s_bytes, cache, k, i));
                }
            }
            cache[idx] = Some(current_max + 1);
            cache[idx].unwrap()
        }
        let mut result_max = 0;
        let mut cache = vec![None; s.len()];
        for i in 0..s.len() {
            result_max = result_max.max(dfs(s.as_bytes(), &mut cache, k as u8, i));
        }
        result_max
    }
}
