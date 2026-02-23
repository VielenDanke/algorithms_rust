pub struct Solution;

impl Solution {
    pub fn reverse_bits(n: i32) -> i32 {
        let mut val = n as u32;
        let mut bits = Vec::new();

        for _ in 0..32 {
            bits.push(val % 2);
            val /= 2;
        }

        let mut result = 0u32;

        for i in 0..32 {
            if bits[i] == 1 {
                result += 2u32.pow((31 - i) as u32);
            }
        }

        result as i32
    }
}