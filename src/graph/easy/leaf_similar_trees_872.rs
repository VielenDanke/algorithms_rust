use std::cell::RefCell;
use std::rc::Rc;

use crate::graph::TreeNode;

pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    inorder(&root1, &mut list1);
    inorder(&root2, &mut list2);
    list1 == list2
}

fn inorder(root: &Option<Rc<RefCell<TreeNode>>>, list: &mut Vec<i32>) {
    if let Some(node) = root {
        inorder(&node.borrow().left, list);
        if node.borrow().left.is_none() && node.borrow().right.is_none() {
            list.push(node.borrow().val);
        }
        inorder(&node.borrow().right, list);
    }
}
