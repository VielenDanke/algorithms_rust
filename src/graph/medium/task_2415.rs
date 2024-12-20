// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use crate::graph::TreeNode;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

pub struct Solution;

impl Solution {
    fn dfs(
        root1: &Option<Rc<RefCell<TreeNode>>>,
        root2: &Option<Rc<RefCell<TreeNode>>>,
        level: i32,
    ) {
        match (root1, root2) {
            (Some(root1_ref), Some(root2_ref)) => {
                let mut root1_node = root1_ref.borrow_mut();
                let mut root2_node = root2_ref.borrow_mut();

                if level % 2 == 0 {
                    let temp = root1_node.val;
                    root1_node.val = root2_node.val;
                    root2_node.val = temp;
                }

                Self::dfs(&root1_node.left, &root2_node.right, level + 1);
                Self::dfs(&root1_node.right, &root2_node.left, level + 1);
            }
            _ => {}
        }
    }

    pub fn reverse_odd_levels_dfs(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root_ref) = &root {
            let root_node = root_ref.borrow();
            Self::dfs(&root_node.left, &root_node.right, 0);
        }
        root
    }

    pub fn reverse_odd_levels_faster(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut queue = VecDeque::new();
        let mut is_odd = false;

        queue.push_back(root.clone().unwrap());

        while !queue.is_empty() {
            if is_odd {
                let mut left = 0;
                let mut right = queue.len() - 1;

                while left < right {
                    if let Ok(mut lt) = queue[left].try_borrow_mut() {
                        if let Ok(mut rt) = queue[right].try_borrow_mut() {
                            (lt.val, rt.val) = (rt.val, lt.val);
                            left += 1;
                            right -= 1;
                        }
                    }
                }
            }
            for _ in 0..queue.len() {
                if let Ok(node) = queue.pop_front().unwrap().try_borrow() {
                    if let Some(left) = node.left.as_ref() {
                        queue.push_back(left.clone());
                    }
                    if let Some(right) = node.right.as_ref() {
                        queue.push_back(right.clone());
                    }
                }
            }
            is_odd = !is_odd;
        }

        root
    }

    pub fn reverse_odd_levels(
        mut root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        let mut level = 0;
        let mut level_map = HashMap::new();

        queue.push_back(root.take().unwrap());

        while !queue.is_empty() {
            let n = queue.len();

            for _ in 0..n {
                if let Ok(mut borrowed_node) = queue.pop_front().unwrap().try_borrow_mut() {
                    level_map
                        .entry(level)
                        .or_insert(Vec::new())
                        .push(borrowed_node.val);

                    if let Some(left) = borrowed_node.left.take() {
                        queue.push_back(left);
                    }
                    if let Some(right) = borrowed_node.right.take() {
                        queue.push_back(right);
                    }
                }
            }
            level += 1;
        }
        for l in 0..level {
            if l % 2 != 0 {
                level_map.entry(l).or_insert(Vec::new()).reverse();
            }
        }
        Self::build_tree_from_level_map(level_map)
    }

    pub fn build_tree_from_level_map(
        level_map: HashMap<i32, Vec<i32>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if level_map.is_empty() {
            return None;
        }

        let root_val = level_map.get(&0).unwrap()[0];
        let root = Rc::new(RefCell::new(TreeNode::new(root_val)));
        let mut queue = VecDeque::new();
        queue.push_back(root.clone());

        let mut level = 0;
        while !queue.is_empty() {
            let n = queue.len();
            level += 1;

            if let Some(level_values) = level_map.get(&level) {
                let mut value_index = 0;
                for _ in 0..n {
                    if let Ok(mut parent_node) = queue.pop_front().unwrap().try_borrow_mut() {
                        if value_index < level_values.len() {
                            let left_child =
                                Rc::new(RefCell::new(TreeNode::new(level_values[value_index])));
                            parent_node.left = Some(left_child.clone());
                            queue.push_back(left_child);
                            value_index += 1;
                        }

                        if value_index < level_values.len() {
                            let right_child =
                                Rc::new(RefCell::new(TreeNode::new(level_values[value_index])));
                            parent_node.right = Some(right_child.clone());
                            queue.push_back(right_child);
                            value_index += 1;
                        }
                    }
                }
            } else {
                break; // No more levels to process
            }
        }
        Some(root)
    }
}
