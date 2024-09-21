pub struct Solution;

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut result = Vec::<i32>::with_capacity(n as usize);
        let mut x = 1;
        result.push(x);
        for _ in 1..n {
            if x * 10 <= n {
                x *= 10;
            } else {
                x += 1;
                while x % 10 == 0 || x > n {
                    x = (x + 9) / 10;
                }
            }
            result.push(x);
        }
        result
    }
}