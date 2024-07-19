pub struct Solution;

impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::new();
        let (m, n) = (matrix.len(), matrix[0].len());

        for i in 0..m {
            let mut min_number = 1 << 30;
            let mut min_idx = 0;
            for j in 0..n {
                if min_number > matrix[i][j] {
                    min_number = matrix[i][j];
                    min_idx = j;
                }
            }
            let mut is_found = true;
            for i in 0..m {
                if min_number < matrix[i][min_idx] {
                    is_found = false;
                    break;
                }
            }
            if is_found {
                result.push(min_number);
            }
        }
        result
    }
}
