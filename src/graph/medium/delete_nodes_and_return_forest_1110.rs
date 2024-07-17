use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use crate::graph::TreeNode;

pub struct Solution;

impl Solution {
    pub fn del_nodes(root: Option<Rc<RefCell<TreeNode>>>, to_delete: Vec<i32>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        fn dfs(
            root: Option<Rc<RefCell<TreeNode>>>,
            v: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
            s: &HashSet<i32>,
            is_root: bool,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(node) = root {
                let mut n = node.borrow_mut();
                let is_future_root = s.contains(&n.val);
                n.left = dfs(n.left.take(), v, s, is_future_root);
                n.right = dfs(n.right.take(), v, s, is_future_root);
                drop(n);
                if is_root && !is_future_root {
                    v.push(Some(node.clone()));
                }
                if !is_future_root {
                    return Some(node)
                }
            }
            None
        }

        let mut result = vec![];
        dfs(root, &mut result, &to_delete.into_iter().collect::<HashSet<i32>>(), true);
        result
    }
}
