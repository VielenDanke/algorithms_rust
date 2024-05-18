use crate::graph::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

impl Solution {
    pub fn distribute_coins(mut root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, moves: &mut i32) -> i32 {
            return if let Some(node) = node.as_ref() {
                if let Ok(mut node) = node.try_borrow_mut() {
                    let left = dfs(node.left.take(), moves);
                    let right = dfs(node.right.take(), moves);
                    *moves += left.abs() + right.abs();
                    node.val + left + right - 1
                } else {
                    0
                }
            } else {
                0
            }
        }
        let mut moves = 0;
        dfs(root, &mut moves);
        moves
    }
}
