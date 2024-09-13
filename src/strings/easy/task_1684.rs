pub struct Solution;

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let arr = allowed.as_bytes()
            .iter()
            .fold(vec![0; 26], |mut a, &b| {
                a[(b - b'a') as usize] += 1;
                a
            });
        words.into_iter()
            .filter(|w| {
                !w.as_bytes().iter().any(|&b| arr[(b - b'a') as usize] == 0)
            })
            .count() as i32
    }
}