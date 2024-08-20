pub struct Solution;

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i64 {
        let mut cache = vec![vec![-1 << 30; points[0].len() + 1]; points.len()];
        Self::pick(&points, 0, -1, &mut cache)
    }

    fn pick(points: &Vec<Vec<i32>>, current_row: usize, previous_col: i32, cache: &mut Vec<Vec<i64>>) -> i64 {
        if current_row == points.len() {
            return 0;
        }
        if cache[current_row][(previous_col + 1) as usize] != -1 << 30 {
            return cache[current_row][(previous_col + 1) as usize];
        }
        let mut max_sum = -1 << 30;
        for j in 0..points[current_row].len() {
            let mut temp_sum = Self::pick(points, current_row + 1, j as i32, cache);
            temp_sum += points[current_row][j] as i64;
            if previous_col > -1 {
                temp_sum -= j.abs_diff(previous_col as usize) as i64;
            }
            max_sum = max_sum.max(temp_sum);
        }
        cache[current_row][(previous_col as usize) + 1] = max_sum;
        max_sum
    }
}