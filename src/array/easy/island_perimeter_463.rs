/// Topics: Array, Depth-First Search, Breadth-First Search, Matrix
struct Solution {}

impl Solution {
    pub fn island_perimeter_iterative(grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 1 {
                    result += 4;
                    if i > 0 && grid[i - 1][j] == 1 {
                        result -= 2;
                    }
                    if j > 0 && grid[i][j - 1] == 1 {
                        result -= 2;
                    }
                }
            }
        }
        result
    }
}

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 1 {
                    return Solution::dfs(&mut Vec::from(grid), i, j);
                }
            }
        }
        -1
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, row: usize, col: usize) -> i32 {
        if Solution::is_boarders_violated(grid, row as i32, col as i32) || grid[row][col] == 0 {
            return 1;
        }
        let current = grid[row][col];

        if current != 1 {
            return 0;
        }
        grid[row][col] = 2;
        let mut result = 0;
        result += Solution::dfs(grid, row + 1, col);
        result += Solution::dfs(grid, row, col + 1);
        result += Solution::dfs(grid, row - 1, col);
        result += Solution::dfs(grid, row, col - 1);
        return result;
    }

    fn is_boarders_violated(grid: &Vec<Vec<i32>>, row: i32, col: i32) -> bool {
        row < 0 || col < 0 || row as usize >= grid.len() || col as usize >= grid[0].len()
    }
}
