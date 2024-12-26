use crate::graph::TreeNode;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn minimum_operations(mut root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = 0;

        let mut queue = VecDeque::new();

        queue.push_back(root.take());

        while !queue.is_empty() {
            let n = queue.len();
            let mut level_values = vec![0; n];

            for i in 0..n {
                if let Some(node) = queue.pop_front() {
                    if let Some(node) = node {
                        if let Ok(mut node) = node.try_borrow_mut() {
                            level_values[i] = node.val;
                            if let Some(node) = node.left.take() {
                                queue.push_back(Some(node));
                            }
                            if let Some(node) = node.right.take() {
                                queue.push_back(Some(node));
                            }
                        }
                    }
                }
            }
            result += Self::total_swaps(level_values);
        }
        result
    }

    fn total_swaps(mut level_values: Vec<i32>) -> i32 {
        let mut swaps = 0;
        let mut sorted_level_values = level_values.clone();
        let n = level_values.len();
        let mut position_map = HashMap::new();

        sorted_level_values.sort_unstable();

        for i in 0..n {
            position_map.entry(level_values[i]).or_insert(i);
        }
        for i in 0..n {
            if level_values[i] != sorted_level_values[i] {
                swaps += 1;

                let current_position = position_map.remove(&sorted_level_values[i]).unwrap();
                position_map.insert(level_values[i], current_position);
                level_values[current_position] = level_values[i];
            }
        }
        swaps
    }
}
