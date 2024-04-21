struct Solution {}

impl Solution {
    pub fn make_good(s: String) -> String {
        if s.len() <= 1 {
            return s;
        }
        let mut str_bytes = s.as_bytes().to_vec();

        let mut i = 0;

        while i < str_bytes.len() - 1 && !str_bytes.is_empty() {
            if str_bytes[i].abs_diff(str_bytes[i + 1]) == 32 {
                str_bytes.remove(i);
                str_bytes.remove(i);
                if i > 0 {
                    i -= 1;
                }
            } else {
                i += 1;
            }
        }
        let mut result = String::new();

        for b in str_bytes {
            result.push(b as char);
        }
        result
    }
}
