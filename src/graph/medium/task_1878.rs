use std::collections::BTreeSet;
use itertools::Itertools;

pub struct Solution;

impl Solution {
    pub fn get_biggest_three(grid: Vec<Vec<i32>>) -> Vec<i32> {
        fn get_rombus_extremes(
            i: usize,
            j: usize,
            m: usize,
            n: usize,
            r: usize,
        ) -> Option<Vec<Vec<usize>>> {
            let top = i - r;
            let bottom = i + r;
            let left = j - r;
            let right = j + r;

            if top > i || bottom >= m || left > j || right >= n {
                None
            } else {
                Some(vec![
                    vec![top, j],
                    vec![bottom, j],
                    vec![i, left],
                    vec![i, right],
                ])
            }
        }

        fn calculate_sum(grid: &Vec<Vec<i32>>, rmb: &Vec<Vec<usize>>) -> i32 {
            let (top_r, top_c) = (rmb[0][0], rmb[0][1]);
            let bottom_r = rmb[1][0];
            let left_r = rmb[2][0];
            let right_r = rmb[3][0];

            let (mut i, mut j, mut result) = (top_r, top_c, 0i32);

            while i < left_r {
                result += grid[i][j];
                i += 1;
                j -= 1;
            }
            while i < bottom_r {
                result += grid[i][j];
                i += 1;
                j += 1;
            }
            while i > right_r {
                result += grid[i][j];
                i -= 1;
                j += 1;
            }
            while i > top_r {
                result += grid[i][j];
                i -= 1;
                j -= 1;
            }
            result
        }
        let m = grid.len();
        let n = grid[0].len();

        let mut set = BTreeSet::new();

        for i in 0..m {
            for j in 0..n {
                set.insert(grid[i][j]);
                for k in 1..m.max(n) {
                    if let Some(extremes) = get_rombus_extremes(i, j, m, n, k) {
                        set.insert(calculate_sum(&grid, &extremes));
                    } else {
                        break;
                    }
                }
            }
        }
        set.into_iter().k_largest(3).collect()
    }
}
