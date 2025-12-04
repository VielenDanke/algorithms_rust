pub struct Solution;

impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        if k % 2 == 0 || k % 5 == 0 {
            return -1;
        }

        let mut remainder = 1 % k;
        let mut length = 1;

        for _ in 0..k {
            if remainder == 0 {
                return length;
            }

            remainder = (remainder * 10 + 1) % k;
            length += 1;
        }

        -1
    }
}