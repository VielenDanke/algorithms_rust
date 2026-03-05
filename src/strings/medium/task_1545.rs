pub struct Solution;

impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        let mut builder = Vec::new();

        builder.push(b'0');

        let mut idx = 1;

        while idx < n && k as usize > builder.len() {
            builder.push(b'1');

            for j in (0..=builder.len() - 2).rev() {
                let inverted_ch = if builder[j] == b'1' { b'0' } else { b'1' };
                builder.push(inverted_ch);
            }
            idx += 1;
        }
        builder[k as usize - 1] as char
    }
}