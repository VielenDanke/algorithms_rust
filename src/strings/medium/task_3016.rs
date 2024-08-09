use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let max_counter = 8;

        let mut bh = word
            .chars()
            .fold(vec![0; 26], |mut acc, ch| {
                acc[(ch as u8 - b'a') as usize] += 1;
                acc
            })
            .into_iter()
            .filter(|&v| v != 0)
            .collect::<BinaryHeap<i32>>();

        let (mut result, mut max_size, mut counter) = (0, 1, max_counter);

        while let Some(letter_count) = bh.pop() {
            result += letter_count * max_size;
            counter -= 1;
            if counter == 0 {
                max_size += 1;
                counter = max_counter;
            }
        }

        result
    }
}