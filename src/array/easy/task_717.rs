pub struct Solution;

impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let mut idx = 0;
        while idx < bits.len() - 1 {
            idx += 1 + bits[idx] as usize;
        }
        idx == bits.len() - 1
    }
}