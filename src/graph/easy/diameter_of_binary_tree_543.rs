use std::cell::RefCell;
use std::rc::Rc;

use crate::graph::TreeNode;

pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, count: &mut i32) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let left = dfs(node.borrow().left.clone(), count);
                let right = dfs(node.borrow().right.clone(), count);
                *count = i32::max(*count, left + right);
                1 + i32::max(left, right)
            }
        }
    }
    let mut count = 0;
    dfs(root, &mut count);
    count
}
