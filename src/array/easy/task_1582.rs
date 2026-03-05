pub struct Solution;

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (mat.len(), mat[0].len());
        let (mut rows, mut cols) = (vec![0; n], vec![0; m]);

        'l: for i in 0..n {
            let mut counter = 0;
            for j in 0..m {
                if mat[i][j] == 1 {
                    counter += 1;
                }
                if counter > 1 {
                    continue 'l;
                }
            }
            rows[i] = 1;
        }
        'l: for j in 0..m {
            let mut counter = 0;
            for i in 0..n {
                if mat[i][j] == 1 {
                    counter += 1;
                }
                if counter > 1 {
                    continue 'l;
                }
            }
            cols[j] = 1;
        }

        let mut result = 0;

        for i in 0..n {
            for j in 0..m {
                if mat[i][j] == 1 && rows[i] == 1 && cols[j] == 1 {
                    result += 1;
                }
            }
        }

        result
    }
}