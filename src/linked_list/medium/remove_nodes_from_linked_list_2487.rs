// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
use crate::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn remove_nodes(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut stack = Vec::new();

        while let Some(node) = head {
            while !stack.is_empty() && stack[stack.len() - 1] < node.val {
                stack.pop();
            }
            stack.push(node.val);
            head = node.next;
        }
        if let Some(val) = stack.pop() {
            let mut node = ListNode::new(val);
            while let Some(val) = stack.pop() {
                node = ListNode { val, next: Some(Box::new(node)) };
            }
            Some(Box::new(node))
        } else {
            None
        }
    }
}
