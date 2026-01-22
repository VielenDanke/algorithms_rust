use std::cmp::min;

pub struct Solution;

impl Solution {
    pub fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());

        // 1. Precompute Prefix Sums with Padding
        // Padding (+1 size) lets us avoid "if index > 0" checks later
        let mut row_sum = vec![vec![0; n + 1]; m];
        let mut col_sum = vec![vec![0; n]; m + 1];

        for i in 0..m {
            for j in 0..n {
                row_sum[i][j + 1] = row_sum[i][j] + grid[i][j];
                col_sum[i + 1][j] = col_sum[i][j] + grid[i][j];
            }
        }

        // Helpers to get sub-segment sums in O(1)
        let get_row_sum = |r: usize, c_start: usize, len: usize| {
            row_sum[r][c_start + len] - row_sum[r][c_start]
        };
        let get_col_sum = |c: usize, r_start: usize, len: usize| {
            col_sum[r_start + len][c] - col_sum[r_start][c]
        };

        // 2. Iterate from largest possible size downwards
        for k in (2..=min(m, n)).rev() {
            // Sliding window of size k
            for i in 0..=(m - k) {
                for j in 0..=(n - k) {

                    // Optimization: Check diagonals first (manual summation)
                    // If diagonals don't match, we skip checking k rows and k cols.
                    let d1: i32 = (0..k).map(|d| grid[i + d][j + d]).sum();
                    let d2: i32 = (0..k).map(|d| grid[i + d][j + k - 1 - d]).sum();

                    if d1 != d2 {
                        continue;
                    }

                    // Check if all Rows match the diagonal sum
                    // We use .all() to break early if a mismatch is found
                    if !(0..k).all(|r| get_row_sum(i + r, j, k) == d1) {
                        continue;
                    }

                    // Check if all Cols match the diagonal sum
                    if (0..k).all(|c| get_col_sum(j + c, i, k) == d1) {
                        return k as i32;
                    }
                }
            }
        }

        1
    }
}