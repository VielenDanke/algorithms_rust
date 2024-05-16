use std::rc::Rc;
use std::cell::RefCell;
use crate::graph::TreeNode;

pub struct Solution;

impl Solution {
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // Leaf nodes 0 - false, 1 - true
        // Non-Leaf nodes 2 - OR, 3 - AND
        if let Some(node) = root {
            if let Ok(mut node) = node.try_borrow_mut() {
                let current_val = node.val;
                return if node.left.is_none() && node.right.is_none() {
                    if current_val == 0 { false } else { true }
                } else {
                    let left = Solution::evaluate_tree(node.left.take());
                    let right = Solution::evaluate_tree(node.right.take());
                    if current_val == 2 {
                        left || right
                    } else {
                        left && right
                    }
                }
            }
        }
        false
    }
}
