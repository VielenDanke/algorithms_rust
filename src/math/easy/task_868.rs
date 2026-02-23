pub struct Solution;

impl Solution {
    pub fn binary_gap(mut n: i32) -> i32 {
        let mut bytes = Vec::new();

        while n > 0 {
            bytes.push(n % 2);
            n = n / 2;
        }
        let mut opt_last_idx = None;
        let mut max_distance = 0;

        for (i, &v) in bytes.iter().enumerate() {
            if v == 1 {
                if let Some(last_idx) = opt_last_idx {
                    max_distance = max_distance.max((i - last_idx) as i32);
                }
                opt_last_idx = Some(i);
            }
        }
        max_distance
    }

    pub fn binary_gap_one_iteration(n: i32) -> i32 {
        let mut opt_last_idx = None;
        let mut max_distance = 0;

        for i in 0..32 {
            if ((n >> i) & 1) > 0 {
                if let Some(last_idx) = opt_last_idx {
                    max_distance = max_distance.max(i - last_idx);
                }
                opt_last_idx = Some(i);
            }
        }
        max_distance
    }
}
