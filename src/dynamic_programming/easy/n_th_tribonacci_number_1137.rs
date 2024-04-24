struct Solution {}

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        if n < 2 {
            return n;
        }
        let mut dp = vec![1; (n + 1) as usize];

        dp[0] = 0;

        (3..=n as usize).into_iter().for_each(|i| dp[i] = dp[i - 1] + dp[i - 2] + dp[i - 3]);

        dp[n as usize]
    }
}
