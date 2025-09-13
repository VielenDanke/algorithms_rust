pub struct Solution;

const VOWELS: [bool; 256] = {
    let mut v = [false; 256];
    v[b'a' as usize] = true;
    v[b'e' as usize] = true;
    v[b'i' as usize] = true;
    v[b'o' as usize] = true;
    v[b'u' as usize] = true;
    v
};

impl Solution {
    pub fn does_alice_win(s: String) -> bool {
        for c in s.chars() {
            if VOWELS[c as usize] {
                return true;
            }
        }
        false
    }
}