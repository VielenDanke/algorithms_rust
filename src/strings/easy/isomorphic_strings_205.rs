use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        fn check(s: &String, t: &String) -> bool {
            let mut map = HashMap::new();
            let (s_bytes, t_bytes) = (s.as_bytes(), t.as_bytes());

            for i in 0..s.len() {
                let s_current = s_bytes[i];
                let t_current = t_bytes[i];
                if let Some(old_value) = map.insert(s_current, t_current) {
                    if old_value != t_current {
                        return false;
                    }
                }
            }
            true
        }
        if s.len() != t.len() {
            false
        } else {
            check(&s, &t) && check(&t, &s)
        }
    }
}
