pub struct Solution;

impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        let mut count = 0;
        for i in 1..=n {
            for j in (i + 1)..=n {
                let sum_sq = i * i + j * j;

                let c_float = (sum_sq as f64).sqrt();
                let c_i = c_float as i32;

                if c_i > n {
                    break;
                }

                if c_i * c_i == sum_sq {
                    count += 2;
                }
            }
        }
        count
    }
}
