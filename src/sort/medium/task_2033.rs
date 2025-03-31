pub struct Solution;

impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut unflatten_grid = grid.iter().flat_map(|row| row.iter()).map(|v| *v).collect::<Vec<i32>>();

        unflatten_grid.sort_unstable();

        let n = unflatten_grid.len();

        let final_common_number = unflatten_grid[n / 2];

        let mut result = 0;

        for num in unflatten_grid {
            if num % x != final_common_number % x {
                return -1;
            }
            result += (final_common_number - num).abs() / x;
        }
        result
    }
}