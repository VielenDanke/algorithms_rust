pub struct Solution;

impl Solution {
    pub fn colored_cells(mut n: i32) -> i64 {
        let mut blue_cells = 1i64;
        let mut addend = 4i64;

        n -= 1;

        while n > 0 {
            blue_cells += addend;
            addend += 4;
            n -= 1;
        }
        blue_cells
    }
}