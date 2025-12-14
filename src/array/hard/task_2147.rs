const MOD: i64 = 1_000_000_007;

pub struct Solution;

impl Solution {

    pub fn number_of_ways(corridor: String) -> i32 {
        let mut chairs = 0i64;
        let mut plants = 0i64;
        let mut result = 1i64;

        for ch in corridor.chars() {
            if ch == 'S' {
                if chairs == 2 {
                    result = (result * (plants + 1)) % MOD;
                    chairs = 0;
                    plants = 0;
                }
                chairs += 1;
            } else if chairs == 2 {
                plants += 1;
            }
        }

        if chairs == 2 {
            result as i32
        } else {
            0
        }
    }
}
