pub struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut start_vec = vec![vec![1]];

        let mut current_row = 2;
        let num_rows = num_rows as usize;

        while current_row <= num_rows {
            let mut current_vec = vec![];
            for i in 0..current_row {
                if i == 0 || i == current_row - 1 {
                    current_vec.push(1);
                } else {
                    current_vec.push(start_vec[current_row - 2][i - 1] + start_vec[current_row - 2][i]);
                }
            }
            start_vec.push(current_vec);
            current_row += 1;
        }
        start_vec
    }
}
