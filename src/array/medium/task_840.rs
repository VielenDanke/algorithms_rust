pub struct Solution;

impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());

        if m < 3 || n < 3 {
            return 0;
        }

        let mut result = 0;

        for i in 0..=m - 3 {
            for j in 0..=n - 3 {
                if Self::is_magic(&grid, i, j) {
                    result += 1;
                }
            }
        }

        result
    }

    fn is_magic(grid: &Vec<Vec<i32>>, i: usize, j: usize) -> bool {
        if grid[i + 1][j + 1] != 5 {
            return false;
        }
        let mut seen = vec![false; 10];

        for k in 0..3 {
            for p in 0..3 {
                let val = grid[i + k][j + p];
                if val < 0 || val > 9 || seen[val as usize] {
                    return false;
                }
                seen[val as usize] = true;
            }
        }

        if grid[i][j] + grid[i][j + 1] + grid[i][j + 2] != 15 {
            return false;
        }
        if grid[i + 1][j] + grid[i + 1][j + 1] + grid[i + 1][j + 2] != 15 {
            return false;
        }
        if grid[i + 2][j] + grid[i + 2][j + 1] + grid[i + 2][j + 2] != 15 {
            return false;
        }
        if grid[i][j] + grid[i + 1][j] + grid[i + 2][j] != 15 {
            return false;
        }
        if grid[i][j] + grid[i + 1][j] + grid[i + 2][j] != 15 {
            return false;
        }
        if grid[i][j + 1] + grid[i + 1][j + 1] + grid[i + 2][j + 1] != 15 {
            return false;
        }
        if grid[i][j + 2] + grid[i + 1][j + 2] + grid[i + 2][j + 2] != 15 {
            return false;
        }
        if grid[i][j] + grid[i + 1][j + 1] + grid[i + 2][j + 2] != 15 {
            return false;
        }
        if grid[i + 2][j] + grid[i + 1][j + 1] + grid[i][j + 2] != 15 {
            return false;
        }
        true
    }
}
