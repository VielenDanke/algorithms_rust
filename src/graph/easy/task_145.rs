use std::cell::RefCell;
use std::rc::Rc;

use crate::graph::TreeNode;

pub struct Solution;

impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn postorder(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
            if let Some(node) = root {
                if let Ok(mut node) = node.try_borrow_mut() {
                    postorder(node.left.take(), result);
                    postorder(node.right.take(), result);
                    result.push(node.val);
                }
            }
        }
        let mut result = Vec::new();
        postorder(root, &mut result);
        result
    }
}