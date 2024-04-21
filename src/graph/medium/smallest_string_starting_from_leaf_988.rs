use crate::graph::*;

struct Solution {}

impl Solution {
    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, letters: &mut Vec<u8>) -> String {
            let mut result = String::new();
            if let Some(node) = root {
                if let Ok(mut node) = node.try_borrow_mut() {
                    letters.push(node.val as u8 + 'a' as u8);
                    if node.left.is_none() && node.right.is_none() {
                        let mut cloned_vector = letters.clone();
                        cloned_vector.reverse();
                        result = String::from_utf8(cloned_vector).unwrap();
                    } else {
                        if node.left.is_some() && node.right.is_some() {
                            result = dfs(node.left.take(), letters).min(dfs(node.right.take(), letters));
                        } else if node.left.is_some() {
                            result = dfs(node.left.take(), letters);
                        } else if node.right.is_some() {
                            result = dfs(node.right.take(), letters);
                        }
                    }
                    letters.pop();
                    return result;
                }
            }
            result
        }
        dfs(root, &mut Vec::new())
    }
}
