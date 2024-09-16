pub struct Solution;

impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut v = Vec::with_capacity(time_points.len() + 1);
        // convert to minutes
        for s in time_points.iter() {
            v.push(s[..2].parse::<i32>().unwrap() * 60 + s[3..].parse::<i32>().unwrap());
        }
        // sort
        v.sort_unstable();
        // add smaller time + 24 * 60 minutes
        v.push(v[0] + 24 * 60);
        // take window of 2, compare and find min
        v.windows(2).map(|w| w[1] - w[0]).min().unwrap()
    }
}