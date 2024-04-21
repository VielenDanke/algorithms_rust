use std::cmp::max;

pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    } else if nums.len() == 1 {
        return nums[0];
    }
    let n = nums.len();

    let mut dp = vec![0i32; n];

    dp[0] = nums[0];
    dp[1] = max(dp[0], nums[1]);

    for i in 2..n {
        dp[i] = max(dp[i - 2] + nums[i], dp[i - 1]);
    }
    dp[n - 1]
}
