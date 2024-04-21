pub struct Solution {}

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut stack = Vec::new();

        let mut s_bytes = s.as_bytes().to_vec();

        for (i, b) in s.as_bytes().to_vec().iter().enumerate() {
            if b == &('(' as u8) {
                stack.push(i);
            } else if b == &(')' as u8) {
                if !stack.is_empty() {
                    stack.remove(stack.len() - 1);
                } else {
                    *s_bytes.get_mut(i).unwrap() = '#' as u8;
                }
            }
        }
        for idx in stack {
            *s_bytes.get_mut(idx).unwrap() = '#' as u8;
        }
        let mut result = String::new();

        for v in s_bytes {
            if v != ('#' as u8) {
                result.push(v as char);
            }
        }
        result
    }
}
