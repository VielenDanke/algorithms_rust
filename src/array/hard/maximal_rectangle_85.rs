struct Solution {}

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let (mut row, mut col) = (matrix.len(), matrix[0].len());

        if row == 1 && col == 1 {
            return if matrix[0][0] == '1' {
                1
            } else {
                0
            }
        }
        let mut h = vec![0; col + 1];
        let mut max_area = 0;

        let mut matrix_clone = Vec::from(matrix);

        for (i, row) in matrix_clone.iter_mut().enumerate() {
            let mut stack = vec![-1i32];
            row.push('0');
            for (j, x) in row.iter().enumerate() {
                if x == &'1' {
                    h[j] += 1;
                } else {
                    h[j] = 0;
                }
                while stack.len() > 1 && (j == col || h[j] < h[stack[stack.len() - 1] as usize]) {
                    let m = stack[stack.len() - 1];
                    stack.pop();
                    let w = j as i32 - stack[stack.len() - 1] - 1;
                    let area = h[m as usize] * w;
                    max_area = max_area.max(area);
                }
                stack.push(j as i32);
            }
        }
        max_area
    }
}
