use std::cell::RefCell;
use std::rc::Rc;

use crate::graph::TreeNode;

struct Solution {}

impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn sum(node: Option<Rc<RefCell<TreeNode>>>, num: i32) -> i32 {
            if let Some(c_node) = node {
                if let Ok(mut borrowed_c_node) = c_node.try_borrow_mut() {
                    let num = num * 10 + borrowed_c_node.val;
                    if borrowed_c_node.left.is_none() && borrowed_c_node.right.is_none() {
                        return num;
                    }
                    sum(borrowed_c_node.left.take(), num) + sum(borrowed_c_node.right.take(), num)
                } else {
                    0
                }
            } else {
                0
            }
        }
        sum(root, 0)
    }

    pub fn sum_numbers_string(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn sum(node: Option<Rc<RefCell<TreeNode>>>, number: &mut String, numbers: &mut Vec<String>) {
            if let Some(c_node) = node {
                if let Ok(mut borrowed_c_node) = c_node.try_borrow_mut() {
                    number.push_str(format!("{}", borrowed_c_node.val).as_str());
                    if borrowed_c_node.left.is_none() && borrowed_c_node.right.is_none() {
                        numbers.push(number.clone());
                    } else {
                        sum(borrowed_c_node.left.take(), number, numbers);
                        sum(borrowed_c_node.right.take(), number, numbers);
                    }
                    number.pop();
                }
            }
        }
        let mut result = Vec::new();
        sum(root, &mut String::from(""), &mut result);
        result.iter().map(|s| s.parse::<i32>().unwrap_or_default()).sum()
    }
}
