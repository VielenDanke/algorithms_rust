pub struct Solution;

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut seconds = 0;

        for i in 1..points.len() {
            seconds += i32::max(
                (points[i][0] - points[i - 1][0]).abs(),
                (points[i][1] - points[i - 1][1]).abs(),
            );
        }

        seconds
    }

    pub fn min_time_to_visit_all_points_declarative(points: Vec<Vec<i32>>) -> i32 {
        points
            .windows(2)
            .map(|pair| {
                (pair[0][0] - pair[1][0])
                    .abs()
                    .max((pair[0][1] - pair[1][1]).abs())
            })
            .sum()
    }
}

