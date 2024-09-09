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
    pub fn spiral_matrix(m: i32, n: i32, head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        let (n, m) = (n as usize, m as usize);

        let mut result = vec![vec![-1; n]; m];

        Self::recursive(0, 0, m-1, n-1, &mut result, head.as_ref());

        result
    }

    fn recursive(x: usize, y: usize, m: usize, n: usize, result: &mut Vec<Vec<i32>>, head: Option<&Box<ListNode>>) {
        if x > m || y > n || head.is_none() {
            return;
        }
        let mut current_head = head;
        for i in y..=n {
            if let Some(node) = current_head {
                result[x][i] = node.val;
                current_head = node.next.as_ref();
            } else {
                return;
            }
        }
        for j in x + 1..=m {
            if let Some(node) = current_head {
                result[j][n] = node.val;
                current_head = node.next.as_ref();
            } else {
                return;
            }
        }
        for i in (y..=n-1).rev() {
            if let Some(node) = current_head {
                result[m][i] = node.val;
                current_head = node.next.as_ref();
            } else {
                return;
            }
        }
        for j in (x + 1..=m - 1).rev() {
            if let Some(node) = current_head {
                result[j][x] = node.val;
                current_head = node.next.as_ref();
            } else {
                return;
            }
        }
        Self::recursive(x + 1, y + 1, m - 1, n - 1, result, current_head);
    }
}