use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
        let mut map: HashMap<(u8, u8), Vec<u8>> = HashMap::new();
        for s in allowed {
            let b = s.as_bytes();
            map.entry((b[0], b[1])).or_default().push(b[2]);
        }

        let bottom_bytes = bottom.into_bytes();
        let mut initial_next_row = Vec::with_capacity(bottom_bytes.len() - 1);

        Self::dfs(&bottom_bytes, &mut initial_next_row, &map)
    }

    fn dfs(row: &[u8], next_row: &mut Vec<u8>, map: &HashMap<(u8, u8), Vec<u8>>) -> bool {
        if row.len() == 1 {
            return true;
        }

        if next_row.len() == row.len() - 1 {
            return Self::dfs(next_row, &mut Vec::with_capacity(next_row.len() - 1), map);
        }

        let idx = next_row.len();
        let key = (row[idx], row[idx + 1]);

        if let Some(tops) = map.get(&key) {
            for &top in tops {
                next_row.push(top);

                if Self::dfs(row, next_row, map) {
                    return true;
                }

                next_row.pop();
            }
        }

        false
    }
}