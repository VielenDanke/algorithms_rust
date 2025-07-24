pub struct Solution;

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut count = 0;
        let mut prev = None;
        let mut res = String::with_capacity(s.len());

        for ch in s.bytes() {
            if prev.is_none() || ch == prev.unwrap() {
                count += 1;
            } else {
                count = 1;
            }
            if count <= 2 {
                res.push(ch as char);
            } 
            prev = Some(ch);
        }

        res
    }
}