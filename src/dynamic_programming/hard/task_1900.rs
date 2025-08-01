use std::cmp::{max, min};

pub struct Solution;

impl Solution {
    pub fn earliest_and_latest(n: i32, first_player: i32, second_player: i32) -> Vec<i32> {
        const MAX_N: usize = 30;
        let mut f = [[[0; MAX_N]; MAX_N]; MAX_N];
        let mut g = [[[0; MAX_N]; MAX_N]; MAX_N];

        let mut first = first_player as usize;
        let mut second = second_player as usize;
        if first > second {
            std::mem::swap(&mut first, &mut second);
        }
        Self::dp(n as usize, first, second, &mut f, &mut g)
    }

    fn dp(
        n: usize,
        first: usize,
        second: usize,
        f: &mut [[[i32; 30]; 30]; 30],
        g: &mut [[[i32; 30]; 30]; 30],
    ) -> Vec<i32> {
        if f[n][first][second] != 0 {
            return vec![f[n][first][second], g[n][first][second]];
        }

        if first + second == n + 1 {
            return vec![1, 1];
        }

        // Symmetric situation handling
        if first + second > n + 1 {
            let v = Self::dp(n, n + 1 - second, n + 1 - first, f, g);
            f[n][first][second] = v[0];
            g[n][first][second] = v[1];
            return v;
        }

        let mut earliest = i32::MAX;
        let mut latest = i32::MIN;
        let n_half = (n + 1) / 2;
        if second <= n_half {
            // All on the left or center
            for i in 0..first {
                for j in 0..(second - first) {
                    let v = Self::dp(n_half, i + 1, i + j + 2, f, g);
                    earliest = min(earliest, v[0]);
                    latest = max(latest, v[1]);
                }
            }
        } else {
            // second on the right
            let s_prime = n + 1 - second;
            let mid = (n - 2 * s_prime + 1) / 2;
            for i in 0..first {
                for j in 0..(s_prime - first) {
                    let v = Self::dp(n_half, i + 1, i + j + mid + 2, f, g);
                    earliest = min(earliest, v[0]);
                    latest = max(latest, v[1]);
                }
            }
        }
        f[n][first][second] = earliest + 1;
        g[n][first][second] = latest + 1;
        vec![f[n][first][second], g[n][first][second]]
    }
}
