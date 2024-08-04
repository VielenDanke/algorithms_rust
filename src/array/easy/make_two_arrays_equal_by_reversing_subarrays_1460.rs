pub struct Solution;

impl Solution {
    pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
        let mut validator = vec![0; 1001];

        if target.len() != arr.len() {
            return false;
        }

        for i in 0..target.len() {
            validator[target[i] as usize] += 1;
            validator[arr[i] as usize] -= 1;
        }

        !validator.iter().any(|v| v != &0)
    }
}