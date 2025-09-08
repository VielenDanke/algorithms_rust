pub struct Solution;

impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let n_usize = n as usize;
        let mut result = vec![0; n_usize];
        let mut k = 1;

        for i in 0..n_usize/2 {
            result[i] = k;
            result[n_usize - i - 1] = k * -1;
            k += 1;
        }
        result
    }
}