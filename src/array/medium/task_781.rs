use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut mpp = HashMap::new();

        for i in answers {
            *mpp.entry(i).or_insert(0) += 1;
        }
        let mut total = 0;

        for (&k, &v) in mpp.iter() {
            total += (v as f32 / (k as f32 + 1.0)).ceil() as i32 * (k + 1);
        }
        total
    }
}