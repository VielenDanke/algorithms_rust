pub struct Solution;

impl Solution {
    pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let (mut x, mut y, mut cur) = (0i64, 0i64, 0i64);
        for d in differences {
            cur += d as i64;
            x = x.min(cur);
            y = y.max(cur);
        }
        ((upper as i64 - lower as i64) - (y - x) + 1).max(0) as i32
    }
}