use std::rc::Rc;
use std::cell::RefCell;
use crate::graph::TreeNode;

pub struct Solution;

impl Solution {
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut arr = Vec::new();

        Self::collect_in_order(root.clone(), &mut arr);

        Self::build_tree(arr)
    }

    fn build_tree(arr: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if arr.is_empty() {
            None
        } else {
            let n = arr.len();
            let middle = n / 2;
            let middle_element = arr[middle];

            let mut node = TreeNode::new(middle_element);

            node.left = Self::build_tree(arr[0..middle].to_vec());
            node.right = Self::build_tree(arr[middle + 1..n].to_vec());

            Some(Rc::new(RefCell::new(node)))
        }
    }

    fn collect_in_order(root: Option<Rc<RefCell<TreeNode>>>, arr: &mut Vec<i32>) {
        if let Some(node_ptr) = root {
            if let Ok(mut node) = node_ptr.try_borrow_mut() {
                Self::collect_in_order(node.left.take(), arr);
                arr.push(node.val);
                Self::collect_in_order(node.right.take(), arr);
            }
        }
    }
}
