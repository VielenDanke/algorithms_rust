use std::cmp::max;
use std::cmp::min;

pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
    points.sort_by(|a, b| a[1].cmp(&b[1]));

    let (mut result, mut end) = (1, points[0][1]);

    for i in 1..points.len() {
        if points[i][0] > end {
            result += 1;
            end = points[i][1];
        }
    }
    result
}

pub fn find_min_arrow_shots_stack(mut points: Vec<Vec<i32>>) -> i32 {
    points.sort_by(|l, r| if l[0] == r[0] { l[1].cmp(&r[1]) } else { l[0].cmp(&r[0]) });

    let mut stack = Vec::new();

    for point in &points {
        if stack.is_empty() || !is_intersect(&stack.get(stack.len() - 1).unwrap(), &point) {
            stack.push(point.to_vec());
        } else {
            let last_elem = stack.remove(stack.len() - 1);
            stack.push(vec![max(point[0], last_elem[0]), min(point[1], last_elem[1])]);
        }
    }
    stack.len() as i32
}

fn is_intersect(from_stack: &Vec<i32>, next: &Vec<i32>) -> bool {
    (next[0] >= from_stack[0] && next[0] <= from_stack[1]) || (next[1] >= from_stack[0] && next[1] <= from_stack[1])
}
