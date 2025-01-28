pub struct Solution;

impl Solution {
    pub fn find_max_fish(mut grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(grid: &mut Vec<Vec<i32>>, row: i32, col: i32) -> i32 {
            let row_u = row as usize;
            let col_u = col as usize;
            
            if row < 0 || col < 0 || row_u >= grid.len() || col_u >= grid[row_u].len() || grid[row_u][col_u] == 0 {
                0
            } else {
                let mut total = grid[row_u][col_u];
                grid[row_u][col_u] = 0;

                let adjacent: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

                for v in adjacent {
                    total += dfs(grid, row + v.0, col + v.1);
                }
                total
            }
        }
        let mut total = 0;

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] > 0 {
                    total = total.max(dfs(&mut grid, i as i32, j as i32));
                }
            }
        }
        total
    }
}