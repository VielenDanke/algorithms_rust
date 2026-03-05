pub struct Solution;

impl Solution {
    pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut trailing_zeros = vec![0; n];

        // 1. Pre-calculate trailing zeros for each row
        for i in 0..n {
            let mut count = 0;
            // Iterate backwards through the row
            for &val in grid[i].iter().rev() {
                if val == 0 {
                    count += 1;
                } else {
                    break;
                }
            }
            trailing_zeros[i] = count;
        }

        let mut total_swaps = 0;

        // 2. Greedy approach to satisfy requirements
        for i in 0..n {
            let required_zeros = (n - 1 - i) as i32;
            let mut found_idx = None;

            // Find the first row from i onwards that satisfies the condition
            for j in i..n {
                if trailing_zeros[j] >= required_zeros {
                    found_idx = Some(j);
                    break;
                }
            }

            // If no such row is found, it's impossible to make the grid valid
            let j = match found_idx {
                Some(idx) => idx,
                None => return -1,
            };

            // 3. "Bubble" the chosen row up to index i
            // This counts the number of adjacent swaps needed
            for k in (i..j).rev() {
                trailing_zeros.swap(k, k + 1);
                total_swaps += 1;
            }
        }

        total_swaps
    }
}