pub struct Solution;

impl Solution {
    pub fn matrix_score(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut score: i32 = (1i32 << (cols - 1)) * (rows as i32);
        for j in 1..cols {
            let count = grid
                .iter()
                .fold(0i32, |acc, row| acc + if row[0] == row[j] { 1 } else { 0 });
            score += count.max(rows as i32 - count) * (1 << (cols - j - 1));
        }
        score
    }

    pub fn matrix_score_brute_force(grid: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());

        for i in 0..rows {
            if grid[i][0] == 0 {
                for j in 0..cols {
                    grid[i][j] = 1 - grid[i][j];
                }
            }
        }
        for j in 1..cols {
            let mut count_zeroes = 0;
            for i in 0..rows {
                if grid[i][j] == 0 {
                    count_zeroes += 1;
                }
            }
            if count_zeroes >= rows - count_zeroes {
                for i in 0..rows {
                    grid[i][j] = 1 - grid[i][j];
                }
            }
        }
        let mut score = 0;

        for i in 0..rows {
            for j in 0..cols {
                let column_score = grid[i][j] << (cols - j - 1);
                score += column_score;
            }
        }
        score
    }
}
