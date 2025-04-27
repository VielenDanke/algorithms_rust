pub struct Solution;

impl Solution {
    pub fn count_good_numbers(n: i64) -> i32 {
        const MOD: u64 = 1_000_000_007;

        fn pow(mut base: u64, mut exp: u64) -> u64 {
            let mut result = 1;
            base %= MOD;
            while exp > 0 {
                if exp % 2 == 1 {
                    result = result * base % MOD;
                }
                base = base * base % MOD;
                exp /= 2;
            }
            result
        }
        let n = n as u64;
        let even_positions = (n + 1) / 2;
        let odd_positions = n / 2;

        let even_ways = pow(5, even_positions);
        let odd_ways = pow(4, odd_positions);

        ((even_ways * odd_ways) % MOD) as i32
    }
}
