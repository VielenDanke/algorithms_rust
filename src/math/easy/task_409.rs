pub struct Solution;

impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let turned_on = turned_on as u32;
        let mut result = Vec::new();

        for h in 0u32..12 {
            for m in 0u32..60 {
                if h.count_ones() + m.count_ones() == turned_on {
                    result.push(format!("{h}:{m:02}"));
                }
            }
        }
        result
    }
}