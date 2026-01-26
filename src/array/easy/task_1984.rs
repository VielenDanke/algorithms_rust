pub struct Solution;

impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();

        nums.windows(k as usize)
            .fold(1 << 30, |a, b| a.min(b[b.len() - 1] - b[0]))
    }
}
