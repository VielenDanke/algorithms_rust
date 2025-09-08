pub struct Solution;

impl Solution {
    pub fn product_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        fn min_powers_of_two(mut n: u32) -> Vec<u32> {
            let mut powers = Vec::new();
            while n > 0 {
                let mut p = 1;
                while p * 2 <= n {
                    p *= 2;
                }
                powers.push(p);
                n -= p;
            }
            powers
        }
        let powers = min_powers_of_two(n as u32);
        let powers_n = powers.len();

        let mut result = vec![];

        for query in queries {
            let mut current_sum = 1;
            for q in query[0]..=query[1] {
                current_sum *= powers[powers_n - q as usize - 1] as i64;
                current_sum %= 1_000_000_007;
            }
            result.push(current_sum);
        }
        result.iter().map(|x| (*x % 1_000_000_007) as i32).collect::<Vec<i32>>()
    }
}