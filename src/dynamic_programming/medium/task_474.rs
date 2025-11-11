pub struct Solution;

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;

        let counts: Vec<(usize, usize)> = strs
            .iter()
            .map(|s| {
                s.as_bytes().iter().fold((0, 0), |(zeroes, ones), &b| {
                    if b == b'0' {
                        (zeroes + 1, ones)
                    } else {
                        (zeroes, ones + 1)
                    }
                })
            })
            .collect();

        let mut memo: Vec<Vec<Vec<Option<i32>>>> = vec![vec![vec![None; n + 1]; m + 1]; strs.len()];

        Self::dp_helper(&counts, m, n, 0, &mut memo)
    }

    fn dp_helper(
        counts: &Vec<(usize, usize)>,
        m: usize,
        n: usize,
        idx: usize,
        memo: &mut Vec<Vec<Vec<Option<i32>>>>,
    ) -> i32 {
        if idx == counts.len() {
            return 0;
        }

        if let Some(cached_val) = memo[idx][m][n] {
            return cached_val;
        }

        let mut max_count = Self::dp_helper(counts, m, n, idx + 1, memo);

        let (zeroes, ones) = counts[idx];

        if m >= zeroes && n >= ones {
            let take_option = 1 + Self::dp_helper(counts, m - zeroes, n - ones, idx + 1, memo);
            max_count = max_count.max(take_option);
        }

        memo[idx][m][n] = Some(max_count);
        max_count
    }
}
