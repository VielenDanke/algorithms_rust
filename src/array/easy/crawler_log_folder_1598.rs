use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn min_operations_only_step(logs: Vec<String>) -> i32 {
        let (stay, move_up) = (String::from("./"), String::from("../"));

        let mut steps = 0;

        for log in logs.iter() {
            if *log == stay {
                continue;
            } else if *log == move_up {
                steps = i32::max(steps - 1, 0);
            } else {
                steps += 1;
            }
        }
        steps
    }

    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut stack = VecDeque::new();

        let (stay, move_up) = (String::from("./"), String::from("../"));

        for log in logs.iter() {
            if *log == stay {
                continue;
            } else if *log == move_up {
                if !stack.is_empty() {
                    stack.pop_back();
                }
            } else {
                stack.push_back(log);
            }
        }
        stack.len() as i32
    }
}
