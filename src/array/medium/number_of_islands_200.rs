struct Solution {}

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        fn dfs(grid: &mut Vec<Vec<char>>, directions: &Vec<(i32, i32)>, row: i32, col: i32) {
            if row < 0 || col < 0 || row >= grid.len() as i32 || col >= grid[row as usize].len() as i32 {
                return;
            }
            if grid[row as usize][col as usize] == '0' {
                return;
            }
            grid[row as usize][col as usize] = '0';

            for dir in directions {
                dfs(grid, directions, row + dir.0, col + dir.1)
            }
        }
        let mut grid_clone = Vec::from(grid);
        let directions = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut result = 0;

        for i in 0..grid_clone.len() {
            for j in 0..grid_clone[i].len() {
                if grid_clone[i][j] == '1' {
                    dfs(&mut grid_clone, &directions, i as i32, j as i32);
                    result += 1;
                }
            }
        }
        result
    }
}
