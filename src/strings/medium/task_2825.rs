pub struct Solution;

impl Solution {
    pub fn can_make_subsequence(source: String, target: String) -> bool {
        let mut target_idx = 0;
        let target_len = target.len();
        let target_bytes = target.as_bytes();

        for curr_char in source.bytes() {
            if target_idx < target_len && (target_bytes[target_idx] as i32 - curr_char as i32 + 26) % 26 <= 1 {
                target_idx += 1;
            }
        }
        target_idx == target_len
    }

    pub fn can_make_subsequence_two_pointers(source: String, target: String) -> bool {
        let src_chars: Vec<char> = source.chars().collect();
        let tgt_chars: Vec<char> = target.chars().collect();
        let tgt_len = tgt_chars.len();

        let mut target_char = tgt_chars[0];
        let mut tgt_idx = 0;

        for src_char in src_chars {
            if src_char == target_char ||
                ((src_char as u8 + 1) as char == target_char) ||
                (src_char == 'z' && target_char == 'a') {
                tgt_idx += 1;
                if tgt_idx == tgt_len {
                    break;
                }
                target_char = tgt_chars[tgt_idx];
            }
        }

        tgt_idx == tgt_len
    }
}