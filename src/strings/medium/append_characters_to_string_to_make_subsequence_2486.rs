pub struct Solution;

impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let (mut left, mut right) = (0, 0);

        let (s_bytes, t_bytes) = (s.as_bytes(), t.as_bytes());

        while left < s_bytes.len() && right < t_bytes.len() {
            while left < s_bytes.len() && s_bytes[left] != t_bytes[right] {
                left += 1;
            }
            if left == s_bytes.len() {
                break;
            }
            left += 1;
            right += 1;
        }
        (t_bytes.len() - right) as i32
    }
}
