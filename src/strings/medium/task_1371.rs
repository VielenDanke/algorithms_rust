pub struct Solution;

impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        let vowels = [b'a', b'e', b'i', b'o', b'u'];
        let mut prv = [i32::MAX; 1 << 5];
        let (mut ans, mut mask) = (0, 0);
        for (i, c) in (0_i32..).zip(s.bytes()) {
            prv[mask] = prv[mask].min(i);
            if let Ok(idx) = vowels.binary_search(&c) {
                mask ^= 1 << idx;
            }
            ans = ans.max((i + 1).saturating_sub(prv[mask]));
        }
        ans
    }

    pub fn find_the_longest_substring_tle(s: String) -> i32 {
        fn is_valid(sub_arr: &[u8], vowel_idx: &Vec<u8>) -> bool {
            let mut alph = vec![0; 26];

            for &b in sub_arr.iter() {
                alph[(b - b'a') as usize] += 1;
            }
            for &idx in vowel_idx.iter() {
                if alph[idx as usize] % 2 != 0 && alph[idx as usize] != 0 {
                    return false;
                }
            }
            true
        }
        let vowel_idx = vec![b'a' - b'a', b'e' - b'a', b'i' - b'a', b'o' - b'a', b'u' - b'a'];

        let mut max_result = 0;

        let s_bytes = s.as_bytes();

        let mut window = 1;

        while window <= s_bytes.len() {
            for i in 0..s_bytes.len() {
                if i + window > s_bytes.len() {
                    break;
                }
                let sub_arr = &s_bytes[i..i + window];

                if is_valid(&sub_arr, &vowel_idx) {
                    let n = sub_arr.len();
                    max_result = max_result.max(n);
                }
            }
            window += 1;
        }
        max_result as i32
    }
}