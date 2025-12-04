use std::collections::HashMap;

pub struct Solution;

impl Solution {

    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        fn gcd(a: i64, b: i64) -> i64 {
            if b == 0 { a } else { gcd(b, a % b) }
        }

        let n = points.len();
        if n < 4 { return 0; }

        const MOD: i64 = 1_000_000_007;
        const INV2: i64 = 500_000_004;

        let mut slope_map: HashMap<(i64, i64), HashMap<i64, i64>> = HashMap::new();
        let mut vector_map: HashMap<(i64, i64), HashMap<i64, i64>> = HashMap::new();

        for i in 0..n {
            for j in (i + 1)..n {
                let x1 = points[i][0] as i64;
                let y1 = points[i][1] as i64;
                let x2 = points[j][0] as i64;
                let y2 = points[j][1] as i64;

                let dx = x2 - x1;
                let dy = y2 - y1;

                let g = gcd(dx.abs(), dy.abs());
                let mut s_dx = dx / g;
                let mut s_dy = dy / g;

                if s_dx < 0 || (s_dx == 0 && s_dy < 0) {
                    s_dx = -s_dx;
                    s_dy = -s_dy;
                }

                let intercept = s_dy * x1 - s_dx * y1;

                *slope_map.entry((s_dx, s_dy))
                    .or_default()
                    .entry(intercept)
                    .or_default() += 1;

                let mut v_dx = dx;
                let mut v_dy = dy;

                if v_dx < 0 || (v_dx == 0 && v_dy < 0) {
                    v_dx = -v_dx;
                    v_dy = -v_dy;
                }

                *vector_map.entry((v_dx, v_dy))
                    .or_default()
                    .entry(intercept)
                    .or_default() += 1;
            }
        }

        let calculate = |groups: HashMap<(i64, i64), HashMap<i64, i64>>| -> i64 {
            let mut total = 0;
            for (_, lines) in groups {
                let mut sum_segments = 0;
                let mut sum_same_line_pairs = 0;

                for &count in lines.values() {
                    sum_segments += count;
                    sum_same_line_pairs = (sum_same_line_pairs + (count * (count - 1)) / 2) % MOD;
                }

                let total_pairs = (sum_segments * (sum_segments - 1)) / 2;
                let valid_pairs = (total_pairs % MOD - sum_same_line_pairs + MOD) % MOD;
                total = (total + valid_pairs) % MOD;
            }
            total
        };

        let trap_count = calculate(slope_map);
        let para_count = calculate(vector_map);

        let para_correction = (para_count * INV2) % MOD;
        let result = (trap_count - para_correction + MOD) % MOD;

        result as i32
    }

}