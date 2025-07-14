#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

/*
let binary = vec![1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0];
    let mut result = 0;

    let len = binary.len();
    for (i, bit) in binary.iter().enumerate() {
        let power = len - 1 - i;
        result += bit * i32::pow(2, power as u32);
    }

    println!("Decimal value: {}", result);
 */

impl Solution {
    pub fn get_decimal_value_bit(mut head: Option<Box<ListNode>>) -> i32 {
        let mut res: i32 = 0;
        while let Some(node) = head {
            res += (res << 1) | node.val;
            head = node.next;
        }
        res
    }

    pub fn get_decimal_value(mut head: Option<Box<ListNode>>) -> i32 {
        let mut res: i32 = 0;
        let to_pow: i32 = 2;
        let mut n = 0;

        let mut new_head = head.clone();

        while let Some(node) = head {
            n += 1;
            head = node.next;
        }
        let mut i = 0;

        while let Some(node) = new_head {
            if node.val == 1 {
                res += to_pow.pow(n - 1 - i);
            }
            i += 1;
            new_head = node.next;
        }
        res
    }
}
