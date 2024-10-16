use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn longest_diverse_string_heap(a: i32, b: i32, c: i32) -> String {
        let mut result = String::with_capacity((a + b + c) as usize);
        let mut heap = BinaryHeap::with_capacity(3);

        if a > 0 {
            heap.push((a, 'a'));
        }
        if b > 0 {
            heap.push((b, 'b'));
        }
        if c > 0 {
            heap.push((c, 'c'));
        }

        let mut total = a + b + c;
        let mut last = None;

        while let Some((mut count, ch)) = heap.pop() {
            result.push(ch);
            count -= 1;
            total -= 1;

            if count > (total - count) * 2 {
                count -= 1;
                total -= 1;
                result.push(ch);
            }

            if let Some(last) = last.take() {
                heap.push(last);
            }

            if count > 0 {
                last = Some((count, ch));
            }
        }
        result
    }

    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        fn backtrack(result: &mut Vec<u8>, a: i32, b: i32, c: i32) -> String {
            if a == 0 && b == 0 && c == 0 {
                String::from_utf8(result.clone()).unwrap()
            } else {
                let mut temp = String::new();
                if (result.len() < 2 || !(result[result.len() - 1] == b'a' && result[result.len() - 2] == b'a')) && a > 0 {
                    result.push(b'a');
                    let current = backtrack(result, a - 1, b, c);
                    if current.len() > temp.len() {
                        temp = current;
                    }
                    result.pop();
                }
                if (result.len() < 2 || !(result[result.len() - 1] == b'b' && result[result.len() - 2] == b'b')) && b > 0 {
                    result.push(b'b');
                    let current = backtrack(result, a, b - 1, c);
                    if current.len() > temp.len() {
                        temp = current;
                    }
                    result.pop();
                }
                if (result.len() < 2 || !(result[result.len() - 1] == b'c' && result[result.len() - 2] == b'c')) && c > 0 {
                    result.push(b'c');
                    let current = backtrack(result, a, b, c - 1);
                    if current.len() > temp.len() {
                        temp = current;
                    }
                    result.pop();
                }
                if result.len() > temp.len() {
                    temp = String::from_utf8(result.clone()).unwrap();
                }
                temp
            }
        }
        backtrack(&mut Vec::new(), a, b, c)
    }
}