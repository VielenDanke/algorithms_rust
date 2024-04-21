use std::cmp::max;

pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    intervals.push(new_interval);
    intervals.sort_by(|left, right| left[0].cmp(&right[0]));

    let mut result: Vec<Vec<i32>> = Vec::new();

    for interval in intervals {
        if result.is_empty() || interval[0] > result[result.len() - 1][1] {
            result.push(interval);
        } else {
            let mut last_interval = result.remove(result.len()-1);
            last_interval[1] = max(interval[1], last_interval[1]);
            result.push(last_interval);
        }
    }
    result
}
