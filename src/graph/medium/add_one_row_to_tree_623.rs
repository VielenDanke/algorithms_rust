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
use std::collections::VecDeque;
use crate::graph::TreeNode;

struct Solution {}

impl Solution {
    pub fn add_one_row(root: Option<Rc<RefCell<TreeNode>>>, val: i32, depth: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if depth < 2 { return Some(Rc::new(RefCell::new(TreeNode { val, left: root, right: None }))); }
        let mut queue = VecDeque::new();

        queue.push_back(root.clone());

        for _ in 2..depth {
            for _ in 0..queue.len() {
                if let Some(n) = queue.pop_front() {
                    if let Some(n) = n {
                        if let Ok(n) = n.try_borrow() {
                            queue.push_back(n.left.clone());
                            queue.push_back(n.right.clone());
                        }
                    }
                }
            }
        }
        while queue.len() > 0 {
            if let Some(n) = queue.pop_front() {
                if let Some(n) = n {
                    if let Ok(mut n) = n.try_borrow_mut() {
                        n.left = Some(Rc::new(RefCell::new(TreeNode { val, left: n.left.take(), right: None })));
                        n.right = Some(Rc::new(RefCell::new(TreeNode { val, left: None, right: n.right.take() })));
                    }
                }
            }
        };
        root
    }
}
