pub struct Solution;

impl Solution {
    const MOD: i64 = 1_000_000_007;

    pub fn num_tilings(n: i32) -> i32 {
        let mut dp = vec![[None, None]; (n + 1) as usize + 2];
        Self::count_dominoes(0, n, false, &mut dp) as i32
    }

    fn count_dominoes(sum: i32, n: i32, is_gap: bool, dp: &mut Vec<[Option<i64>; 2]>) -> i64 {
        if sum > n {
            return 0;
        }
        if sum == n {
            return if is_gap { 0 } else { 1 };
        }

        let gap_idx = if is_gap { 1 } else { 0 };
        
        if let Some(value) = dp[sum as usize][gap_idx] {
            return value;
        }

        let result = if is_gap {
            (Self::count_dominoes(sum + 1, n, false, dp)
                + Self::count_dominoes(sum + 1, n, true, dp))
                % Self::MOD
        } else {
            (Self::count_dominoes(sum + 1, n, false, dp)
                + Self::count_dominoes(sum + 2, n, false, dp)
                + 2 * Self::count_dominoes(sum + 2, n, true, dp))
                % Self::MOD
        };

        dp[sum as usize][gap_idx] = Some(result);
        result
    }
}
