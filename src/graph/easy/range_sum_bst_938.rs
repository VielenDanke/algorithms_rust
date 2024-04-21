use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::graph::TreeNode;

pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    let mut stack: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();

    stack.push_back(root);

    let mut sum: i32 = 0;

    while let Some(stack_object) = stack.pop_front() {
        let node = stack_object.as_ref().unwrap().replace(TreeNode::new(0));
        if node.val >= low && node.val <= high {
            sum += node.val;
        }
        if node.val > low && node.left.is_some() {
            stack.push_back(node.left);
        }
        if node.val < high && node.right.is_some() {
            stack.push_back(node.right);
        }
    }
    sum
}

pub fn range_sum_bst_recursive(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    if let Some(r) = root {
        let r = r.borrow();
        if r.val >= low && r.val <= high {
            return r.val +
                range_sum_bst_recursive(r.right.clone(), low, high) +
                range_sum_bst_recursive(r.left.clone(), low, high);
        } else if r.val < low { return range_sum_bst_recursive(r.right.clone(), low, high); } else if r.val > high { return range_sum_bst_recursive(r.left.clone(), low, high); }
    }
    0
}
