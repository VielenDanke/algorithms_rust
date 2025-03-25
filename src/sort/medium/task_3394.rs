pub struct Solution;

impl Solution {
    pub fn check_valid_cuts(_: i32, rectangles: Vec<Vec<i32>>) -> bool {
        let mut x_intervals: Vec<(i32, i32)> = rectangles.iter().map(|r| (r[0], r[2])).collect();
        let mut y_intervals: Vec<(i32, i32)> = rectangles.iter().map(|r| (r[1], r[3])).collect();

        Self::check(&mut x_intervals) || Self::check(&mut y_intervals)
    }

    fn check(intervals: &mut Vec<(i32, i32)>) -> bool {
        intervals.sort_unstable_by_key(|&(start, _)| start);

        let mut sections = 0;
        let mut max_end = intervals[0].1;

        for &(start, end) in intervals.iter() {
            if max_end <= start {
                sections += 1;
            }
            max_end = max_end.max(end);
        }

        sections >= 2
    }
}