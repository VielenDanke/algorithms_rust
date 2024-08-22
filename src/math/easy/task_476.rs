pub struct Solution;

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let (mut i, mut j) = (0, 0);

        while i < num {
            i += i32::pow(2, j);
            j += 1;
        }
        i - num
    }
}