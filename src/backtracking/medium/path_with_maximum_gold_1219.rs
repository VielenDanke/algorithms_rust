pub struct Solution;

impl Solution {
    pub fn get_maximum_gold(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] != 0 {
                    result = result.max(Solution::backtrack(&mut grid, i as i32, j as i32, 0));
                }
            }
        }
        result
    }

    fn backtrack(grid: &mut Vec<Vec<i32>>, row: i32, col: i32, current_value: i32) -> i32 {
        if Solution::is_size_violated(grid, row, col) {
            return 0;
        }
        let row_idx = row as usize;
        let col_idx = col as usize;
        if grid[row_idx][col_idx] == 0 {
            return current_value;
        }
        let gold = grid[row_idx][col_idx];
        grid[row_idx][col_idx] = 0;
        let mut result = 0;
        result = result.max(Solution::backtrack(grid, row + 1, col, current_value + gold));
        result = result.max(Solution::backtrack(grid, row, col + 1, current_value + gold));
        result = result.max(Solution::backtrack(grid, row - 1, col, current_value + gold));
        result = result.max(Solution::backtrack(grid, row, col - 1, current_value + gold));
        grid[row_idx][col_idx] = gold;
        result

    }

    fn is_size_violated(grid: &Vec<Vec<i32>>, row: i32, col: i32) -> bool {
        return row < 0 || col < 0 || row as usize >= grid.len() || col as usize >= grid[row as usize].len()
    }
}
