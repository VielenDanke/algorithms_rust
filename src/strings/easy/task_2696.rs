pub struct Solution;

impl Solution {
    pub fn min_length(s: String) -> i32 {
        let ab = vec![b'A', b'B'];
        let cd = vec![b'C', b'D'];

        let mut s_bytes = s.as_bytes().to_vec();

        let mut is_match = true;

        while is_match && !s_bytes.is_empty() {
            is_match = false;

            for mut i in 0..s_bytes.len() - 1 {
                if ab[0] == s_bytes[i] && ab[1] == s_bytes[i + 1] {
                    is_match = true;
                    s_bytes[i] = b'*';
                    s_bytes[i + 1] = b'*';
                    continue;
                }
                if cd[0] == s_bytes[i] && cd[1] == s_bytes[i + 1] {
                    is_match = true;
                    s_bytes[i] = b'*';
                    s_bytes[i + 1] = b'*';
                }
            }
            s_bytes = s_bytes.iter().map(|ch| *ch).filter(|ch| *ch != b'*').collect::<Vec<u8>>();
        }
        s_bytes.len() as i32
    }
}