pub struct Solution;

impl Solution {
    pub fn count_permutations(complexity: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = complexity.len();

        if complexity.iter().skip(1).any(|&c| c <= complexity[0]) {
            return 0;
        }

        (1..=n - 1).fold(1_i64, |acc, i| (acc * (i as i64)) % MOD) as i32
    }
}
