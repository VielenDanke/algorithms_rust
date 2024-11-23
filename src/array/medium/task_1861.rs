pub struct Solution;

impl Solution {
    pub fn rotate_the_box(mut matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let row = matrix.len();
        let col = matrix[0].len();

        for i in 0..row {
            let mut total_stones = 0;
            let mut start = 0;

            for j in 0..col {
                if matrix[i][j] == '#' {
                    total_stones += 1;
                } else if matrix[i][j] == '*' && total_stones > 0 {
                    Self::move_stones(&mut matrix, i, start, j, &mut total_stones);
                    start = j + 1;
                }
            }
            if total_stones > 0 {
                Self::move_stones(&mut matrix, i, start, col, &mut total_stones);
            }
        }
        Self::rotate_matrix(matrix, row, col)
    }

    fn move_stones(matrix: &mut Vec<Vec<char>>, i: usize, start: usize, end: usize, total_stones: &mut i32) {
        for j in (start..end).rev() {
            if total_stones > &mut 0 {
                matrix[i][j] = '#';
                *total_stones -= 1;
            } else if matrix[i][j] != '*' {
                matrix[i][j] = '.';
            }
        }
    }

    fn rotate_matrix(matrix: Vec<Vec<char>>, row: usize, col: usize) -> Vec<Vec<char>> {
        let mut rotated_matrix: Vec<Vec<char>> = vec![vec![' '; row]; col];

        for i in 0..row {
            for j in 0..col {
                rotated_matrix[j][row - 1 - i] = matrix[i][j];
            }
        }
        rotated_matrix
    }
}