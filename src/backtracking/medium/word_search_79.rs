struct Solution {}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        if board.len() == 0 {
            return false;
        }
        let mut mut_board: Vec<Vec<char>> = Vec::new();
        board.clone_into(&mut mut_board);

        for i in 0..mut_board.len() {
            for j in 0..mut_board[i].len() {
                if board[i][j] == word.as_bytes()[0] as char && Solution::traverse(&mut mut_board, word.as_bytes(), i, j, 0) {
                    return true;
                }
            }
        }
        false
    }

    fn traverse(board: &mut Vec<Vec<char>>, word: &[u8], row: usize, col: usize, idx: usize) -> bool {
        if idx == word.len() {
            return true;
        }
        if !Solution::is_valid_coordinates(board, row, col) {
            return false;
        }
        let current = board[row][col];

        if current == '.' || current != word[idx] as char {
            return false;
        }
        board[row][col] = '.';

        let is_exists = Solution::traverse(board, word, row + 1, col, idx + 1) ||
            Solution::traverse(board, word, row, col + 1, idx + 1) ||
            Solution::traverse(board, word, row - 1, col, idx + 1) ||
            Solution::traverse(board, word, row, col - 1, idx + 1);

        board[row][col] = current;

        is_exists
    }

    fn is_valid_coordinates(board: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
        row >= 0 && col >= 0 && row < board.len() && col < board[row].len()
    }
}
