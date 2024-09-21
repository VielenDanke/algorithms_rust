pub struct Solution;

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(n as usize);
        let mut x = 1;

        for _ in 1..=n {
            result.push(x);

            if x * 10 <= n {
                x *= 10;
            } else {
                while x % 10 == 9 || x >= n {
                    x /= 10;
                }
                x += 1;
            }
        }
        result
    }
}