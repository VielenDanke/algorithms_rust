use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::graph::TreeNode;

pub struct Solution;

impl Solution {
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut nodes = HashMap::new();
        let mut all_nodes = HashSet::new();

        for current_description in descriptions.iter() {
            nodes.entry(current_description[0])
                .or_insert(Some(Rc::new(RefCell::new(TreeNode::new(current_description[0])))));
            nodes.entry(current_description[1])
                .or_insert(Some(Rc::new(RefCell::new(TreeNode::new(current_description[1])))));
            all_nodes.insert(current_description[0]);
            all_nodes.insert(current_description[1]);
        }

        for current_description in descriptions.iter() {
            let parent_option = nodes.get(&current_description[0]).unwrap();
            let child_option = nodes.get(&current_description[1]).unwrap();
            all_nodes.remove(&current_description[1]);

            if let Some(parent) = parent_option.as_ref() {
                if current_description[2] == 1 {
                    parent.borrow_mut().left = child_option.clone();
                } else {
                    parent.borrow_mut().right = child_option.clone();
                }
            }
        }

        nodes.remove(&all_nodes.iter().last().unwrap()).unwrap()
    }
}
