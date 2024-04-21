use crate::linked_list::ListNode;

pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut temp_head = &head;

    let mut n = 0;

    while temp_head.is_some() {
        temp_head = &temp_head.as_ref().unwrap().next;
        n += 1;
    }
    temp_head = &head;
    for _i in 0..n / 2 {
        temp_head = &temp_head.as_ref().unwrap().next;
    }
    temp_head.clone()
}

pub fn middle_node_fast_slow(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut fast, mut slow) = (&head, &head);

    while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
        slow = &slow.as_ref().unwrap().next;
        fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
    }
    slow.clone()
}
