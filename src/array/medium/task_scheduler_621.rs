use std::cmp::max;

pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
    let mut counter = vec![0i32; 26];
    let mut max_count = 0;

    for task in &tasks {
        counter[*task as usize - 'A' as usize] += 1;
        max_count = max(max_count, counter[*task as usize - 'A' as usize]);
    }
    let mut result = (max_count - 1) * (n + 1);

    for i in 0..26 {
        if counter[i] == max_count {
            result += 1;
        }
    }
    max(result, tasks.len() as i32)
}
