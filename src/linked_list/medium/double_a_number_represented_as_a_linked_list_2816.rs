use crate::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode { val: 0, next: head }));
        let mut prev_box = head.as_mut().unwrap();
        while let Some(curr_box) = prev_box.next.as_mut() {
            let v = curr_box.val * 2;
            curr_box.val = v % 10;
            prev_box.val += v / 10;
            prev_box = curr_box
        }
        if head.as_ref().unwrap().val < 1 {
            head.unwrap().next
        } else {
            head
        }
    }
}
