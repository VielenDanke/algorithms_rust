pub struct Solution;

impl Solution {
    pub fn has_same_digits(s: String) -> bool {
        let mut s_bytes = s.as_bytes().to_vec();

        while s_bytes.len() > 2 {
            for i in 0..s_bytes.len() - 1 {
                s_bytes[i] = (s_bytes[i] + s_bytes[i + 1]) % 10;
            }
            s_bytes.pop();
        }
        s_bytes[0] == s_bytes[1]
    }
}