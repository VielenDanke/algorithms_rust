pub struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }
        let mut alph_s1 = vec![0; 26];
        let mut alph_s2 = vec![0; 26];

        for ch in s1.as_bytes().iter() {
            alph_s1[(ch - b'a') as usize] += 1;
        }
        let bytes_s2 = s2.as_bytes();

        for i in 0..s1.len() {
            alph_s2[(bytes_s2[i] - b'a') as usize] += 1;
        }
        for i in s1.len()..s2.len() {
            if alph_s1 == alph_s2 {
                return true;
            }
            alph_s2[(bytes_s2[i] - b'a') as usize] += 1;
            alph_s2[(bytes_s2[i - s1.len()] - b'a') as usize] -= 1;
        }
        alph_s1 == alph_s2
    }
}