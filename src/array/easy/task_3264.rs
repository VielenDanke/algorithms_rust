use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn get_final_state(mut nums: Vec<i32>, mut k: i32, multiplier: i32) -> Vec<i32> {
        let mut heap = nums.iter().enumerate().map(|(i, &v)| Element::new(v, i)).collect::<BinaryHeap<Element>>();

        while k > 0 && !heap.is_empty() {
            let lowest = heap.pop().unwrap();
            heap.push(Element::new(lowest.number * multiplier, lowest.idx));
            k -= 1;
        }
        for elem in heap.into_iter() {
            nums[elem.idx] = elem.number;
        }
        nums
    }

    pub fn get_final_state_no_struct(mut nums: Vec<i32>, mut k: i32, multiplier: i32) -> Vec<i32> {
        let mut heap = nums
            .iter()
            .enumerate()
            .map(|(i, &v)| Reverse((v, i)))
            .collect::<BinaryHeap<Reverse<(i32, usize)>>>();

        for _ in 0..k {
            if let Some(Reverse((v, i))) = heap.pop() {
                heap.push(Reverse((v * multiplier, i)));
            }
        }
        while let Some(Reverse((v, i))) = heap.pop() {
            nums[i] = v;
        }
        nums
    }
}

#[derive(Debug, Clone)]
struct Element {
    number: i32,
    idx: usize
}

impl Element {
    fn new(number: i32, idx: usize) -> Self {
        Self { number, idx }
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number && self.idx == other.idx
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.number == other.number {
            return other.idx.partial_cmp(&self.idx);
        }
        other.number.partial_cmp(&self.number)
    }
}

impl Eq for Element {}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}
