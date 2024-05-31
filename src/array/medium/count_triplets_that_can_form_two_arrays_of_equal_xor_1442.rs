pub struct Solution;

impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let mut result = 0i32;
        let mut n = arr.len();
        for i in 0..n {
            let mut val = arr[i];
            for k in i+1..n {
                val ^= arr[k];
                if val == 0 {
                    result += (k - i) as i32;
                }
            }
        }
        result
    }
}
