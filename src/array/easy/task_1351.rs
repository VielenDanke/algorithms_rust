pub struct Solution;

impl Solution {
    pub fn count_negatives_fastest(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len() as i32, grid[0].len() as i32);
        let mut i = m - 1;
        let mut j = 0i32;
        let mut result = 0i32;

        while i >= 0 && j < n {
            if grid[i as usize][j as usize] < 0 {
                result += n - j;
                i -= 1;
            } else {
                j += 1;
            }
        }

        result
    }

    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        fn binary_search(arr: &Vec<i32>, mut left: i32, mut right: i32) -> i32 {
            while left < right {
                let mid = (left + right) / 2;

                if arr[mid as usize] >= 0 {
                    left = mid + 1;
                } else if arr[mid as usize] < 0 {
                    right = mid;
                }
            }
            left
        }

        let mut result = 0;

        for arr in grid.iter() {
            let left_idx = binary_search(arr, 0, arr.len() as i32 - 1);

            if arr[left_idx as usize] < 0 {
                result += arr.len() as i32 - left_idx;
            }
        }

        result
    }

    pub fn count_negatives_brute_force(grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] < 0 {
                    result += 1;
                }
            }
        }

        result
    }
}