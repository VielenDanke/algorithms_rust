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
    pub fn remove_leaf_nodes(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
        root.as_ref().and_then(|node| {
            let mut node_ref = node.borrow_mut();
            node_ref.left = Self::remove_leaf_nodes(node_ref.left.take(), target);
            node_ref.right = Self::remove_leaf_nodes(node_ref.right.take(), target);
            if node_ref.left.is_none() && node_ref.right.is_none() && node_ref.val == target {
                None
            } else {
                Some(node.clone())
            }
        })
    }

    pub fn remove_leaf_nodes_brute(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.as_ref() {
            if let Ok(mut node) = node.try_borrow_mut() {
                return if node.left.is_none() && node.right.is_none() {
                    if node.val == target {
                        None
                    } else {
                        root.clone()
                    }
                } else {
                    if node.left.is_some() {
                        node.left = Self::remove_leaf_nodes_brute(node.left.take(), target);
                    }
                    if node.right.is_some() {
                        node.right = Self::remove_leaf_nodes_brute(node.right.take(), target);
                    }
                    if node.left.is_none() && node.right.is_none() && node.val == target {
                        None
                    } else {
                        root.clone()
                    }
                }
            }
        }
        None
    }
}
