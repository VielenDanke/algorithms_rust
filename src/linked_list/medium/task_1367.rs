// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
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
use std::cell::{Ref, RefCell};
use crate::linked_list::ListNode;
use crate::graph::TreeNode;

pub struct Solution;

impl Solution {
    pub fn is_sub_path(mut head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut res = false;
        let mut head_vec = vec![];
        while let Some(node) = head {
            head_vec.push(node.val);
            head = node.next;
        }
        Self::dfs(root.as_ref(), &head_vec, &mut vec![], &mut res);
        res
    }

    pub fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, head: &Vec<i32>, mut tmp: &mut Vec<i32>, res: &mut bool) {
        if *res {
            return;
        }
        if let Some(node) = root {
            if let Ok(node) = node.try_borrow() {
                tmp.push(node.val);
                if tmp.windows(head.len()).any(|window| window == head) {
                    *res = true;
                    return;
                }
                Self::dfs(node.right.as_ref(), head, tmp, res);
                Self::dfs(node.left.as_ref(), head, tmp, res);
                tmp.remove(tmp.len() - 1);
            }
        }
    }

    pub fn is_sub_path_tle(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn in_order(mut node: Option<&Rc<RefCell<TreeNode>>>, next: Option<&Box<ListNode>>, head: Option<&Box<ListNode>>) -> bool {
            if next.is_none() {
                return true;
            }
            if let Some(node) = node {
                if let Ok(node) = node.try_borrow() {
                    if node.val == next.unwrap().val {
                        if in_order(node.left.as_ref(), next.unwrap().next.as_ref(), head) {
                            return true;
                        }
                        if in_order(node.right.as_ref(), next.unwrap().next.as_ref(), head) {
                            return true;
                        }
                    } else {
                        if in_order(node.left.as_ref(), head.unwrap().next.as_ref(), head) {
                            return true;
                        }
                        if in_order(node.right.as_ref(), head.unwrap().next.as_ref(), head) {
                            return true;
                        }
                    }
                }
            }
            false
        }
        in_order(root.as_ref(), head.as_ref(), head.as_ref())
    }
}