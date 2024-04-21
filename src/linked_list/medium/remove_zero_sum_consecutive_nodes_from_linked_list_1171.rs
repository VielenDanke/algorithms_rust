use crate::linked_list::ListNode;

pub fn remove_zero_sum_sublists(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut stack = vec![];

    while head.is_some() {
        let current_node = head.unwrap();
        if current_node.val == 0 {} else if stack.is_empty() {
            stack.push(current_node.val);
        } else {
            let idx = extract_sum_index(&stack, current_node.val);
            if idx >= 0 {
                stack = stack[0..(idx as usize)].to_vec();
            } else {
                stack.push(current_node.val);
            }
        }
        head = current_node.next;
    }
    if stack.is_empty() {
        return None;
    }
    re_inject_into_linked_list(stack)
}

fn re_inject_into_linked_list(stack: Vec<i32>) -> Option<Box<ListNode>> {
    let mut new = ListNode::new(0);
    let mut n = &mut new;

    for val in stack.into_iter().map(ListNode::new).map(Box::new) {
        n.next = Some(val);
        n = n.next.as_mut()?;
    }
    n.next = None;
    new.next
}

fn extract_sum_index(stack: &Vec<i32>, current_val: i32) -> i32 {
    let mut temp_sum = 0;
    let mut idx = -1;

    for (i, v) in stack.iter().enumerate().rev() {
        temp_sum += *v;
        if temp_sum + current_val == 0 {
            idx = i as i32;
            break;
        }
    }
    idx
}
