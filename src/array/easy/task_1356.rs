pub struct Solution;

impl Solution {
    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        arr.sort_unstable_by(|&a, &b| {
            let a_ones = a.count_ones();
            let b_ones = b.count_ones();
            if a_ones == b_ones {
                a.cmp(&b)
            } else {
                a_ones.cmp(&b_ones)
            }
        });
        arr
    }
}
