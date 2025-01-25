use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        // Map to store the index of each number in the arr
        let mut num_to_index: HashMap<i32, usize> = HashMap::new();

        for (i, &num) in arr.iter().enumerate() {
            num_to_index.insert(num, i);
        }

        let mut result = i32::MAX;
        let num_rows = mat.len();
        let num_cols = mat[0].len();

        // Check for the earliest row to be completely painted
        for row in 0..num_rows {
            let mut last_element_index = i32::MIN;
            for col in 0..num_cols {
                let index_val = *num_to_index.get(&mat[row][col]).unwrap() as i32;
                last_element_index = last_element_index.max(index_val);
            }
            // Update result with the minimum index where this row is fully painted
            result = result.min(last_element_index);
        }

        // Check for the earliest column to be completely painted
        for col in 0..num_cols {
            let mut last_element_index = i32::MIN;
            for row in 0..num_rows {
                let index_val = *num_to_index.get(&mat[row][col]).unwrap() as i32;
                last_element_index = last_element_index.max(index_val);
            }
            // Update result with the minimum index where this column is fully painted
            result = result.min(last_element_index);
        }

        result
    }

    pub fn first_complete_index_counter(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let row_len = mat.len();
        let col_len = mat[0].len();

        let mut num_to_position = HashMap::new();

        for i in 0..row_len {
            for j in 0..col_len {
                num_to_position.insert(mat[i][j], (i, j));
            }
        }
        let mut row_counter = vec![0; row_len];
        let mut col_counter = vec![0; col_len];

        for (i, v) in arr.iter().enumerate() {
            if let Some((row, col)) = num_to_position.get(v) {
                row_counter[*row] += 1;
                col_counter[*col] += 1;

                if row_counter[*row] == row_len || col_counter[*col] == col_len {
                    return i as i32;
                }
            }
        }
        -1
    }

    pub fn first_complete_index_brute_force(arr: Vec<i32>, mut mat: Vec<Vec<i32>>) -> i32 {
        let mut num_to_pos: HashMap<i32, (usize, usize)> = HashMap::new();

        for (row, row_vec) in mat.iter().enumerate() {
            for (col, &value) in row_vec.iter().enumerate() {
                num_to_pos.insert(value, (row, col));
            }
        }

        for (i, &num) in arr.iter().enumerate() {
            if let Some(&(row, col)) = num_to_pos.get(&num) {
                let mat_row = &mut mat[row];

                mat_row[col] = -mat_row[col];

                if Self::check_row(row, &mat) || Self::check_column(col, &mat) {
                    return i as i32;
                }
            }
        }
        -1
    }

    fn check_row(row: usize, mat: &Vec<Vec<i32>>) -> bool {
        mat[row].iter().all(|&x| x < 0)
    }

    fn check_column(col: usize, mat: &Vec<Vec<i32>>) -> bool {
        mat.iter().all(|row| row[col] < 0)
    }
}
