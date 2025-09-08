pub struct Solution;

impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        fn does_contain_zeroes(mut n: i32) -> bool {
            while n > 0 {
                if n % 10 == 0 {
                    return true;
                }
                n /= 10;
            }
            false
        }
        for i in 1..n {
            let j = n - i;
            if !does_contain_zeroes(i) && !does_contain_zeroes(j) {
                return vec![i, j];
            }
        }
        vec![]
    }
}