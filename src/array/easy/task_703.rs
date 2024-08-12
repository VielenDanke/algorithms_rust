use std::collections::BinaryHeap;

struct KthLargest {
    container: BinaryHeap<std::cmp::Reverse<i32>>,
    k: usize,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let k = k as usize;
        let mut container = nums
            .into_iter()
            .map(|n| std::cmp::Reverse(n))
            .collect::<BinaryHeap<_>>();
        for _ in k..container.len() {
            container.pop();
        }
        Self { container, k }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.container.push(std::cmp::Reverse(val));
        if self.container.len() > self.k {
            self.container.pop();
        }
        self.container.peek().unwrap().0
    }
}
/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */