use std::cell::RefCell;
use std::rc::Rc;
use crate::graph::TreeNode;

pub struct Solution;

impl Solution {
    pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::traverse(root.clone(), 0);
        root
    }

    fn traverse(node_opt: Option<Rc<RefCell<TreeNode>>>, val: i32) -> i32 {
        if let Some(node_ptr) = node_opt {
            if let Ok(mut node) = node_ptr.try_borrow_mut() {
                node.val += Self::traverse(node.right.clone(), val);

                return Self::traverse(node.left.clone(), node.val);
            }
        }
        val
    }
}
