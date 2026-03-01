pub struct Solution;

impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        const MODULO: i64 = 1_000_000_007;
        let mut result = 0i64;
        for i in 1..=n {
            let mut temp = i;
            while temp > 0 {
                temp /= 2;
                result *= 2;
            }
            result = (result + i as i64) % MODULO;
        }
        result as i32
    }
}
