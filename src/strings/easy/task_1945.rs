use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn get_lucky(s: String, mut k: i32) -> i32 {
        let mut lucky = 0;
        for b in s.bytes() {
            let b = (b - b'a' + 1) as i32;
            if b > 9 {
                lucky += b % 10;
                lucky += b / 10;
            } else {
                lucky += b;
            }
        }
        while k > 1 {
            let mut l = lucky;
            lucky = 0;
            while l > 0 {
                lucky += l % 10;
                l /= 10;
            }
            k -= 1;
        }
        lucky
    }

    pub fn get_lucky_shorter(s: String, mut k: i32) -> i32 {
        let mut s = s.chars().map(|c| (c as u8 - b'a' + 1).to_string()).collect::<Vec<String>>().join("");
        for _ in 0..k {
            s = s.chars().map(|c| c as i32 - '0' as i32).sum::<i32>().to_string();
        }
        s.parse::<i32>().unwrap()
    }
}