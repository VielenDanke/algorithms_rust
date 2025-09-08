pub struct Solution;

impl Solution {
    pub fn sort_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let width = grid[0].len();
        let height = grid.len();
        let size = width.min(height);

        for i in 0..height {
            let mut values = Vec::with_capacity(size);
            for j in 0..height - i {
                values.push(grid[i + j][j]);
            }
            values.sort_unstable_by_key(|&x| -x);
            for j in 0..height - i {
                grid[i + j][j] = values[j];
            }
        }

        for i in 1..width {
            let mut values = Vec::with_capacity(size);
            for j in 0..width - i {
                values.push(grid[j][i + j]);
            }
            values.sort_unstable();
            for j in 0..width - i {
                grid[j][i + j] = values[j];
            }
        }

        grid
    }
}
