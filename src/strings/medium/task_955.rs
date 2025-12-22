pub struct Solution;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let n = strs.len();
        let m = strs[0].len();
        let mut deletions = 0;

        let mut is_sorted = vec![false; n - 1];

        let strs_bytes = strs
            .iter()
            .map(|s| s.as_bytes().to_vec())
            .collect::<Vec<Vec<u8>>>();

        for j in 0..m {
            let mut keep_column = true;

            for i in 0..n - 1 {
                if !is_sorted[i] {
                    if strs_bytes[i][j] > strs_bytes[i + 1][j] {
                        keep_column = false;
                        break;
                    }
                }
            }

            if keep_column {
                for i in 0..n - 1 {
                    if !is_sorted[i] && strs_bytes[i][j] < strs_bytes[i + 1][j] {
                        is_sorted[i] = true;
                    }
                }
            } else {
                deletions += 1;
            }
        }
        deletions
    }
}
