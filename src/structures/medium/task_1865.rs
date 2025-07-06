use std::collections::HashMap;

/**
 * Your FindSumPairs object will be instantiated and called as such:
 * let obj = FindSumPairs::new(nums1, nums2);
 * obj.add(index, val);
 * let ret_2: i32 = obj.count(tot);
 */
struct FindSumPairs {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
    counter: HashMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindSumPairs {
    fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        let mut counter = HashMap::new();

        for &v in nums2.iter() {
            *counter.entry(v).or_insert(0) += 1;
        }

        FindSumPairs {
            nums1,
            nums2,
            counter,
        }
    }

    fn add(&mut self, index: i32, val: i32) {
        let current_value = self.nums2[index as usize];
        self.counter
            .entry(current_value)
            .and_modify(|v| *v -= 1)
            .or_insert(0);
        self.nums2[index as usize] = current_value + val;
        self.counter
            .entry(current_value + val)
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }

    fn count(&self, tot: i32) -> i32 {
        let mut counter = 0;
        for i in 0..self.nums1.len() {
            let current_val = self.nums1[i];

            if let Some(&v) = self.counter.get(&(tot - current_val)) {
                if v > 0 {
                    counter += v;
                }
            }
        }
        counter
    }
}
