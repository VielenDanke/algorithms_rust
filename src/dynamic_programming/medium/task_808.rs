use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn soup_servings(n: i32) -> f64 {
        if n > 4800 {
            return 1.0;
        }

        let m = (n + 24) / 25; // аналог ceil(n / 25)
        let mut dp: HashMap<(i32, i32), f64> = HashMap::new();
        dp.insert((0, 0), 0.5);

        fn calculate_dp(dp: &HashMap<(i32, i32), f64>, i: i32, j: i32) -> f64 {
            let get = |a: i32, b: i32| -> f64 {
                *dp.get(&(a.max(0), b.max(0))).unwrap_or(&0.0)
            };
            (get(i - 4, j)
                + get(i - 3, j - 1)
                + get(i - 2, j - 2)
                + get(i - 1, j - 3)) / 4.0
        }

        for k in 1..=m {
            dp.insert((0, k), 1.0);
            dp.insert((k, 0), 0.0);
            for j in 1..=k {
                let a = calculate_dp(&dp, j, k);
                let b = calculate_dp(&dp, k, j);
                dp.insert((j, k), a);
                dp.insert((k, j), b);
            }
            if let Some(val) = dp.get(&(k, k)) {
                if *val > 1.0 - 1e-5 {
                    return 1.0;
                }
            }
        }

        *dp.get(&(m, m)).unwrap_or(&0.0)
    }
}