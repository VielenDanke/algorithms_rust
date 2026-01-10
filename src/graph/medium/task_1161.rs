use crate::graph::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::new();

        if let Some(node) = root {
            queue.push_back(node.clone());
        } else {
            return -1;
        }

        let mut result_level = 1;
        let mut max_value = -1 << 30;
        let mut level = 0;

        while !queue.is_empty() {
            let n = queue.len();
            let mut level_sum = 0;
            level += 1;

            for _ in 0..n {
                if let Some(node) = queue.pop_front() {
                    if let Ok(node) = node.try_borrow() {
                        level_sum += node.val;

                        if node.left.is_some() {
                            queue.push_back(node.left.clone().unwrap())
                        }
                        if node.right.is_some() {
                            queue.push_back(node.right.clone().unwrap())
                        }
                    }
                }
            }

            if level_sum > max_value {
                max_value = level_sum;
                result_level = level;
            }
        }

        result_level
    }
}