pub struct Solution;

impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        let s_bytes = s.as_bytes();
        let mut score = 0;
        for i in 1..s_bytes.len() {
            score += s_bytes[i].abs_diff((s_bytes[i-1])) as i32;
        }
        score
    }
}
