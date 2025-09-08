pub struct Solution;

const SUDOKU_SIDE: usize = 9;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        Self::are_rows_valid(&board)
            && Self::are_columns_valid(&board)
            && Self::are_squares_valid(&board)
    }

    fn are_rows_valid(board: &Vec<Vec<char>>) -> bool {
        for i in 0..SUDOKU_SIDE {
            let mut numbers = [0u8; SUDOKU_SIDE + 1];
            for j in 0..SUDOKU_SIDE {
                let c = board[i][j];
                if c != '.' {
                    let idx = (c as u8 - b'0') as usize;
                    numbers[idx] += 1;
                    if numbers[idx] > 1 {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn are_columns_valid(board: &Vec<Vec<char>>) -> bool {
        for i in 0..SUDOKU_SIDE {
            let mut numbers = [0u8; SUDOKU_SIDE + 1];
            for j in 0..SUDOKU_SIDE {
                let c = board[j][i];
                if c != '.' {
                    let idx = (c as u8 - b'0') as usize;
                    numbers[idx] += 1;
                    if numbers[idx] > 1 {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn are_squares_valid(board: &Vec<Vec<char>>) -> bool {
        for i in (0..SUDOKU_SIDE).step_by(3) {
            for j in (0..SUDOKU_SIDE).step_by(3) {
                if !Self::is_square_valid(board, i, j) {
                    return false;
                }
            }
        }
        true
    }

    fn is_square_valid(board: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
        let mut numbers = [0u8; SUDOKU_SIDE + 1];
        for i in row..row + 3 {
            for j in col..col + 3 {
                let c = board[i][j];
                if c != '.' {
                    let idx = (c as u8 - b'0') as usize;
                    numbers[idx] += 1;
                    if numbers[idx] > 1 {
                        return false;
                    }
                }
            }
        }
        true
    }
}