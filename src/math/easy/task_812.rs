pub struct Solution;

impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        fn area(p: &Vec<i32>, q: &Vec<i32>, r: &Vec<i32>) -> f64 {
            let px = p[0] as f64;
            let py = p[1] as f64;
            let qx = q[0] as f64;
            let qy = q[1] as f64;
            let rx = r[0] as f64;
            let ry = r[1] as f64;

            let area_val = (px * qy + qx * ry + rx * py) - (py * qx + qy * rx + ry * px);

            0.5 * f64::abs(area_val)
        }

        let n = points.len();
        let mut answer = 0f64;

        for i in 0..n {
            for j in i + 1..n {
                for k in j + 1..n {
                    answer = answer.max(area(&points[i], &points[j], &points[k]))
                }
            }
        }
        answer
    }
}