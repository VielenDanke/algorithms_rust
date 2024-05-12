pub struct Solution;

impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        let n_incr = 3;

        for row in 0..=(grid.len() - n_incr) {
            result.push(Vec::new());

            for col in 0..=(grid[row].len() - n_incr) {
                let mut current_max = 0;

                for inner_row in row..row + n_incr {
                    for inner_col in col..col + n_incr {
                        current_max = current_max.max(grid[inner_row][inner_col]);
                    }
                }
                result[row].push(current_max);
            }
        }
        result
    }
}
