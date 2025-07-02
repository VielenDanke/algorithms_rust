pub struct Solution;

impl Solution {
    pub fn max_distance(s: String, k: i32) -> i32 {
        let mut cnt = [0i32; 4]; // [N, E, S, W]
        let mut res = 0;
        let mut total = 0;

        for c in s.bytes() {
            match c {
                b'N' => cnt[0] += 1,
                b'E' => cnt[1] += 1,
                b'S' => cnt[2] += 1,
                b'W' => cnt[3] += 1,
                _ => continue,
            }
            total += 1;

            let current = (cnt[1] - cnt[3]).abs() + (cnt[0] - cnt[2]).abs();
            let dist = (current + 2 * k).min(total);
            res = res.max(dist);
        }

        res
    }
}