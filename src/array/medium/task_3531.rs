use std::cmp::{max, min};

pub struct Solution;

impl Solution {
    pub fn count_covered_buildings(n: i32, buildings: Vec<Vec<i32>>) -> i32 {
        let size = (n + 1) as usize;

        let mut row_min_c = vec![i32::MAX; size];
        let mut row_max_c = vec![i32::MIN; size];
        let mut col_min_r = vec![i32::MAX; size];
        let mut col_max_r = vec![i32::MIN; size];

        for b in &buildings {
            let r = b[0] as usize;
            let c = b[1] as usize;
            let r_i32 = b[0];
            let c_i32 = b[1];

            row_min_c[r] = min(row_min_c[r], c_i32);
            row_max_c[r] = max(row_max_c[r], c_i32);

            col_min_r[c] = min(col_min_r[c], r_i32);
            col_max_r[c] = max(col_max_r[c], r_i32);
        }

        let mut covered_count = 0;

        for b in &buildings {
            let r = b[0] as usize;
            let c = b[1] as usize;
            let r_i32 = b[0];
            let c_i32 = b[1];

            let has_horizontal = c_i32 > row_min_c[r] && c_i32 < row_max_c[r];

            let has_vertical = r_i32 > col_min_r[c] && r_i32 < col_max_r[c];

            if has_horizontal && has_vertical {
                covered_count += 1;
            }
        }

        covered_count
    }

    pub fn count_covered_buildings_brute_force(n: i32, buildings: Vec<Vec<i32>>) -> i32 {
        let n_usize = n as usize + 1;
        let mut grid = vec![vec![0; n_usize]; n_usize];

        // Mark building locations on the grid
        for building in buildings.iter() {
            grid[building[0] as usize][building[1] as usize] = 1;
        }

        let mut covered_count = 0;
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        fn dfs(grid: &Vec<Vec<i32>>, r: i32, c: i32, dr: i32, dc: i32, n: i32) -> bool {
            if r < 0 || r >= n || c < 0 || c >= n {
                return false;
            }

            if grid[r as usize][c as usize] == 1 {
                return true;
            }

            dfs(grid, r + dr, c + dc, dr, dc, n)
        }

        for i in 0..n {
            for j in 0..n {
                let mut current_covered = 0;

                if grid[i as usize][j as usize] == 1 {
                    for &(dr, dc) in &directions {
                        if dfs(&grid, i + dr, j + dc, dr, dc, n_usize as i32) {
                            current_covered += 1;
                        }
                    }
                }

                if current_covered == directions.len() as i32 {
                    covered_count += 1;
                }
            }
        }

        covered_count
    }
}
