pub struct Solution {}

impl Solution {
    pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        if n == 1 {
            return *grid.get(0).unwrap_or(&Vec::new()).get(0).unwrap_or(&0);
        }
        let mut memo = vec![vec![0; n]; n];

        (0..n).into_iter().for_each(|col| {
            memo[n - 1][col] = grid[n - 1][col];
        });
        for row in (0..=n-2).rev() {
            for col in 0..n {
                let mut next_minimum = 1 << 30;
                for next_row_col in 0..n {
                    if next_row_col != col {
                        next_minimum = next_minimum.min(memo[row + 1][next_row_col]);
                    }
                }
                memo[row][col] = grid[row][col] + next_minimum;
            }
        }
        let mut answer = 1 << 30;
        (0..n).into_iter().for_each(|col| {
            answer = answer.min(memo[0][col]);
        });
        answer
    }
}
