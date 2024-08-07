use std::rc::Rc;
use std::cell::RefCell;
use crate::graph::TreeNode;

pub struct Solution;

impl Solution {
    pub fn get_directions(root: Option<Rc<RefCell<TreeNode>>>, start_value: i32, dest_value: i32) -> String {
        fn dfs(root: &TreeNode, arr: &mut Vec<u8>, target: i32) -> bool {
            if root.val == target {
                return true;
            }
            for (ch, c) in [&root.left, &root.right].iter().zip([b'L', b'R']) {
                if let Some(ch) = ch {
                    arr.push(c);
                    if dfs(&ch.borrow(), arr, target) {
                        return true;
                    }
                    arr.pop();
                }
            }
            false
        }

        let mut start_arr = Vec::new();
        dfs(&root.as_ref().unwrap().borrow(), &mut start_arr, start_value);

        let mut end_arr = Vec::new();
        dfs(&root.as_ref().unwrap().borrow(), &mut end_arr, dest_value);

        let prf_len = start_arr.iter().zip(&end_arr).take_while(|(a, b)| a == b).count();

        let push_up = "U".repeat(start_arr.len() - prf_len);
        let push_bottom = String::from_utf8_lossy(&end_arr[prf_len..]).to_string();

        format!("{push_up}{push_bottom}")
    }
}
