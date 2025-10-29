pub struct Solution;

impl Solution {
    pub fn smallest_number(n: i32) -> i32 {
        let mut result = 0;
        for i in 0..n {
            result += 2i32.pow(i as u32);
            if result >= n {
                return result;
            }
        }
        -1
    }
}