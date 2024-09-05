pub struct Solution;

impl Solution {
    pub fn missing_rolls(mut rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        // rolls - m observations
        // mean - total_rolls.sum() / total_rolls.len()
        // n - expected n's
        // prepare expected
        // mean = total_rolls.sum() / total_rolls.len()
        // total_rolls.sum() = mean * total_rolls.len() where total_rolls = rolls.len() + n

        let expected_sum = mean * (rolls.len() as i32 + n);

        let mut left_sum = expected_sum - rolls.iter().sum::<i32>();

        if left_sum < n || left_sum > n * 6 {
            return vec![]
        }
        let reminder = left_sum % n;

        let mut n_rolls = vec![left_sum / n; n as usize];

        for i in 0..reminder as usize {
            n_rolls[i] += 1;
        }
        n_rolls
    }
}