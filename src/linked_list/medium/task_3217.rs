use std::collections::HashSet;
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
    pub fn modified_list(nums: Vec<i32>, mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let set: HashSet<_> = nums.into_iter().collect();
        let mut dummy = ListNode { next: head, val: 0 };
        let mut curr = &mut dummy;
        while let Some(next_box) = curr.next.as_mut() {
            if set.contains(&next_box.val) {
                curr.next = next_box.next.take()
            } else {
                curr = curr.next.as_mut()?
            }
        }
        dummy.next
    }
}