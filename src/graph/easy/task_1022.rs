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
use std::cell::RefCell;
use crate::graph::TreeNode;

pub struct Solution;

impl Solution {
    pub fn sum_root_to_leaf(mut root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, mut curr: i32) -> i32 {
            if node.is_none() {
                0
            } else {
                let unwrapped_node = node.unwrap();
                let mut borrowed_node = unwrapped_node.borrow_mut();

                curr = curr * 2 + borrowed_node.val;

                if borrowed_node.left.is_none() && borrowed_node.right.is_none() {
                    curr
                } else {
                    dfs(borrowed_node.left.take(), curr) + dfs(borrowed_node.right.take(), curr)
                }
            }
        }
        dfs(root.take(), 0)
    }
}