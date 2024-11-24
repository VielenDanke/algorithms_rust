pub struct Solution;

impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut negative_count = 0;
        let mut min_abs = i32::MAX;
        let mut total_sum: i64 = 0;

        for row in matrix.iter() {
            for &val in row.iter() {
                if val < 0 {
                    negative_count += 1;
                }
                min_abs = min_abs.min(val.abs());
                total_sum += val.abs() as i64;
            }
        }
        if negative_count % 2 == 1 {
            total_sum - (2 * min_abs) as i64
        } else {
            total_sum
        }
    }
}