pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut letters = vec![0i32; 256];

        let mut bytes = s.bytes();

        while let Some(b) = bytes.next() {
            letters[(b - b'a') as usize] += 1;
        }
        let (mut result, mut add) = (0, 0);

        for letter in letters.iter() {
            if *letter % 2 == 0 {
                result += *letter;
            } else {
                result += *letter - 1;
                add = 1;
            }
        }
        result + add
    }
}
