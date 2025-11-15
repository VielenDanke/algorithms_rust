pub struct Solution;

impl Solution {
    pub fn range_add_queries_prefix_sum(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut diff = vec![vec![0; n + 1]; n + 1];

        for q in queries {
            let (row1, col1, row2, col2) =
                (q[0] as usize, q[1] as usize, q[2] as usize, q[3] as usize);
            diff[row1][col1] += 1;
            diff[row2 + 1][col1] -= 1;
            diff[row1][col2 + 1] -= 1;
            diff[row2 + 1][col2 + 1] += 1;
        }

        let mut result = vec![vec![0; n]; n];

        for i in 0..n {
            for j in 0..n {
                let x1 = if i == 0 { 0 } else { result[i - 1][j] };
                let x2 = if j == 0 { 0 } else { result[i][j - 1] };
                let x3 = if i == 0 || j == 0 {
                    0
                } else {
                    result[i - 1][j - 1]
                };
                result[i][j] = diff[i][j] + x1 + x2 - x3;
            }
        }
        result
    }

    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = vec![vec![0; n as usize]; n as usize];

        for query in queries {
            for i in query[0]..=query[2] {
                for j in query[1]..=query[3] {
                    result[i as usize][j as usize] += 1;
                }
            }
        }
        result
    }
}
