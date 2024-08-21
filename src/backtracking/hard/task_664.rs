pub struct Solution;

impl Solution {
    pub fn strange_printer(s: String) -> i32 {
        const MAX_LEN: usize = 101;

        let mut s_arr = Vec::new();

        for &b in s.as_bytes() {
            if s_arr.is_empty() || s_arr[s_arr.len() - 1] != b {
                s_arr.push(b);
            }
        }
        let n = s_arr.len();

        let mut cache = vec![vec![None; n]; n];

        fn backtrack(s: &Vec<u8>, left: usize, right: usize, cache: &mut Vec<Vec<Option<i32>>>) -> i32 {
            if left < 0 || right < 0 || left > right || left > MAX_LEN || right > MAX_LEN {
                return 0;
            }
            if cache[left][right].is_some() {
                return cache[left][right].unwrap();
            }
            let mut temp = backtrack(s, left, right - 1, cache) + 1;

            for i in left..right {
                if s[i] == s[right] {
                    temp = temp.min(backtrack(s, left, i - 1, cache) + backtrack(s, i, right - 1, cache));
                }
            }
            cache[left][right] = Some(temp);
            temp
        }
        return backtrack(&s_arr, 0, n - 1, &mut cache);
    }
}