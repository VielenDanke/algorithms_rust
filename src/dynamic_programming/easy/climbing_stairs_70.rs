pub fn climb_stairs(n: i32) -> i32 {
    let mut dp = vec![1i32, 2];

    for i in 2..n {
        dp.push(dp[(i - 1) as usize] + dp[(i - 2) as usize]);
    }
    dp[(n - 1) as usize]
}
