pub struct Solution;

impl Solution {
    pub fn min_number_operations(target: Vec<i32>) -> i32 {
        let n = target.len();
        let mut result = target[0];
        for i in 1..n {
            result += (target[i] - target[i - 1]).max(0);
        }
        result
    }
}
