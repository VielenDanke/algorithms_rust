use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn kth_smallest_prime_fraction(mut arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut k = k as usize;
        let mut heap = BinaryHeap::with_capacity(k+1);
        while k > 0 && arr.len() > 1 {
            let denominator = arr.pop().unwrap();
            for numerator in arr.iter().copied().take(k) {
                heap.push(Fraction {
                    numerator,
                    denominator,
                });
                if heap.len() == heap.capacity() {
                    heap.pop();
                }
            }
            k -= 1;
        }
        heap.pop().map(Fraction::into_vec).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Copy)]
struct Fraction {
    numerator: i32,
    denominator: i32,
}

impl Fraction {
    fn into_vec(self) -> Vec<i32> {
        vec![self.numerator, self.denominator]
    }
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        (self.numerator * other.denominator).eq(&(self.denominator * other.numerator))
    }
}

impl Eq for Fraction {}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.numerator * other.denominator).cmp(&(self.denominator * other.numerator))
    }
}
