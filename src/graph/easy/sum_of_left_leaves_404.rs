use std::cell::RefCell;
use std::collections::VecDeque;
use std::error::Error;
use std::process;
use std::rc::Rc;

use crate::graph::TreeNode;

struct Solution {}

impl Solution {
    pub fn sum_of_left_leaves_dfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
            if let Some(n) = node {
                if let Ok(mut c_node) = n.try_borrow_mut() {
                    return if c_node.left.is_none() && c_node.right.is_none() && is_left {
                        c_node.val
                    } else {
                        dfs(c_node.left.take(), true) + dfs(c_node.right.take(), false)
                    };
                }
            }
            0
        }
        dfs(root, false)
    }

    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn bfs(node: Option<Rc<RefCell<TreeNode>>>) -> Result<i32, Box<dyn Error>> {
            let mut queue = VecDeque::new();

            queue.push_back((node, false));

            let mut total_sum = 0;

            while !queue.is_empty() {
                let (node, is_left) = queue.pop_front().unwrap();

                if let Some(v) = node {
                    let c_node = v.try_borrow_mut()?;
                    if is_left && c_node.left.is_none() && c_node.right.is_none() {
                        total_sum += c_node.val;
                    }
                    queue.push_back((c_node.left.take(), true));
                    queue.push_back((c_node.right.take(), false));
                }
            }
            Ok(total_sum)
        }
        bfs(root).unwrap_or_else(|err| {
            eprintln!("Error calculating sum of left leaves");
            process::exit(1);
        })
    }

    pub fn sum_of_left_leaves_unsafe(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = Vec::new();

        queue.push((&root, false));

        let mut total_sum = 0;

        while !queue.is_empty() {
            let (node, is_left) = queue.remove(0);

            if let Some(v) = node {
                let c_node = v.as_ptr();
                unsafe {
                    if is_left && (*c_node).left.is_none() && (*c_node).right.is_none() {
                        total_sum += (*c_node).val;
                    }
                    if (*c_node).left.is_some() {
                        queue.push((&(*c_node).left, true));
                    }
                    if (*c_node).right.is_some() {
                        queue.push((&(*c_node).right, false));
                    }
                }
            }
        }
        total_sum
    }
}
