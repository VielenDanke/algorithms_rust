use std::collections::HashSet;

pub struct Solution;

impl Solution {

    pub fn judge_square_sum_binary_search(c: i32) -> bool {
        let mut a = 0i64;
        let c = c as i64;

        while a * a <= c {
            let b = c - a * a;
            if Self::binary_search(0, b, b) {
                return true;
            }
            a += 1;
        }
        false
    }

    fn binary_search(s: i64, e: i64, n: i64) -> bool {
        if s > e {
            return false;
        }
        let mid = s + (e - s) / 2;
        if mid * mid == n {
            return true;
        }
        if mid * mid > n {
            return Self::binary_search(s, mid - 1, n);
        }
        return Self::binary_search(mid + 1, e, n);
    }

    pub fn judge_square_sum_sqrt(c: i32) -> bool {
        let mut a = 0;
        let c = c as i64;

        while a * a <= c {
            let b = ((c - a * a) as f64).sqrt();
            if b.floor() == b {
                return true;
            }
            a += 1;
        }
        false
    }

    pub fn judge_square_sum_tle(c: i32) -> bool {
        let mut a = 0;
        let c = c as i64;
        while a * a <= c {
            let b = c - (a * a);
            let mut i = 1;
            let mut sum = 0;
            while sum < b {
                sum += i;
                i += 2;
            }
            if sum == b {
                return true;
            }
            a += 1;
        }
        false
    }

    pub fn judge_square_sum_brute_force(c: i32) -> bool {
        let mut squares = HashSet::new();
        let c = c as u128;

        for i in 0..=c {
            if i * i > c {
                return false;
            }

            squares.insert(i * i);

            if squares.contains(&(c - i * i)) {
                return true;
            }
        }
        return false;
    }
}
