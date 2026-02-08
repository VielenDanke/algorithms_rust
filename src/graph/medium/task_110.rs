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
use crate::graph::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if root.is_none() {
                return 0;
            }
            let rc_unwrapped = root.unwrap();
            let mut rc = rc_unwrapped.borrow_mut();

            if rc.left.is_none() && rc.right.is_none() {
                return 1;
            }

            let left = dfs(rc.left.take());
            let right = dfs(rc.right.take());

            if left == -1 || right == -1 {
                -1
            } else if (left - right).abs() > 1 {
                -1
            } else {
                left.max(right) + 1
            }
        }

        dfs(root) != -1
    }
}