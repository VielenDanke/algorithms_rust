pub struct Solution;

impl Solution {
    pub fn find_kth_number(n: i32, mut k: i32) -> i32 {
        fn count_number_with_prefix(prefix: i64, n: i64) -> i64 {
            let (mut first_number, mut next_number) = (prefix, prefix + 1);
            let mut total_count = 0;

            while first_number <= n {
                total_count += (n + 1).min(next_number) - first_number;
                first_number *= 10;
                next_number *= 10;
            }
            total_count
        }
        let (n, mut k) = (n as i64, k as i64);
        let mut current_prefix = 1;
        k -= 1;

        while k > 0 {
            let count = count_number_with_prefix(current_prefix, n);
            if k >= count {
                current_prefix += 1;
                k -= count;
            } else {
                current_prefix *= 10;
                k -= 1;
            }
        }
        current_prefix as i32
    }

    pub fn find_kth_number_tle(n: i32, k: i32) -> i32 {
        let k = k as i64;
        let n = n as i64;

        let mut x = 1;

        for _ in 1..k {
            if x * 10 <= n {
                x *= 10;
            } else {
                while x % 10 == 9 || x >= n {
                    x /= 10;
                }
                x += 1;
            }
        }
        x as i32
    }
}