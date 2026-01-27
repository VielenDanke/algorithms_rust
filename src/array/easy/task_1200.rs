pub struct Solution;

impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort_unstable();

        let min_diff = arr
            .windows(2)
            .map(|window| (window[0] - window[1]).abs())
            .reduce(|l, r| l.min(r))
            .unwrap();

        arr.windows(2)
            .filter(|window| (window[0] - window[1]).abs() == min_diff)
            .map(|window| window.to_vec())
            .collect()
    }
}
