use std::rc::Rc;
use std::cell::RefCell;
use crate::graph::TreeNode;

pub struct Solution;

impl Solution {
    pub fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32, result: &mut i32) -> Vec<i32> {
            let unwrapped_node = root.unwrap();
            let mut n = unwrapped_node.borrow_mut();

            if n.left.is_none() && n.right.is_none() {
                return vec![1];
            }
            let mut possible = Vec::new();

            if n.left.is_some() {
                let left_nodes = dfs(n.left.take(), distance, result);
                possible.extend(left_nodes);
            }
            if n.right.is_some() {
                let right_nodes = dfs(n.right.take(), distance, result);

                for &l1 in possible.iter() {
                    for &l2 in right_nodes.iter() {
                        if l1 + l2 <= distance {
                            *result += 1;
                        }
                    }
                }
                possible.extend(right_nodes);
            }
            possible.iter().map(|&v| v + 1).collect::<Vec<i32>>()
        }

        let mut result = 0;

        dfs(root, distance, &mut result);

        result
    }
}
