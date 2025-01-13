pub struct Solution;

impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        fn validate(s: &[u8], locked: &[u8], symbol: u8) -> bool {
            let (mut block, mut free, n) = (0i32, 0i32, s.len() as i32);
            let (start, incr) = if symbol == b'(' {
                (0i32, 1)
            } else {
                (n - 1, -1)
            };
            let mut i = start;

            while i >= 0 && i < n && block + free >= 0 {
                let idx = i as usize;
                if locked[idx] == b'1' {
                    block += if s[idx] == symbol { 1 } else { -1 };
                } else {
                    free += 1;
                }
                i += incr;
            }

            block.abs() <= free
        }
        s.len() % 2 == 0 && validate(s.as_bytes(), locked.as_bytes(), b'(') && validate(s.as_bytes(), locked.as_bytes(), b')')
    }
}
