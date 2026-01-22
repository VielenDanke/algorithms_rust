use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn minimum_pair_removal(mut nums: Vec<i32>) -> i32 {
        let mut counter = 0;
        while !nums.is_empty() {
            if nums.is_sorted() {
                return counter;
            }
            let mut min_sum = 1 << 30;
            for i in 0..nums.len() - 1 {
                min_sum = min_sum.min(nums[i] + nums[i + 1]);
            }
            for i in 0..nums.len() - 1 {
                if min_sum == nums[i] + nums[i + 1] {
                    counter += 1;
                    nums[i] = min_sum;
                    nums.remove(i + 1);
                    break;
                }
            }
        }
        0
    }

    pub fn minimum_pair_removal_different_approach(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 2 {
            return 0;
        }

        // 1. Simulate a Doubly Linked List using arrays
        // right[i] points to the element to the right of i
        // left[i] points to the element to the left of i
        let mut right: Vec<usize> = (1..=n).collect(); // n indicates "end of list"
        let mut left: Vec<usize> = (0..n).map(|i| if i == 0 { usize::MAX } else { i - 1 }).collect();

        // Track which nodes are currently valid (not merged away)
        let mut valid = vec![true; n];

        // 2. Track "unsorted" count to avoid O(N) is_sorted() checks
        let mut unsorted_count = 0;
        for i in 0..n - 1 {
            if nums[i] > nums[i + 1] {
                unsorted_count += 1;
            }
        }

        if unsorted_count == 0 {
            return 0;
        }

        // 3. Min-Heap to store (sum, index).
        // We use Reverse because Rust's BinaryHeap is a Max-Heap.
        // Storing (sum, index) ensures that if sums are equal, we pick the smaller index (leftmost).
        let mut heap = BinaryHeap::new();
        for i in 0..n - 1 {
            heap.push(Reverse((nums[i] as i64 + nums[i + 1] as i64, i)));
        }

        let mut ops = 0;

        while let Some(Reverse((sum, i))) = heap.pop() {
            // If the node i is gone, or the neighbor to its right is gone/changed, this heap entry is stale.
            let j = right[i];
            if !valid[i] || j >= n || !valid[j] {
                continue;
            }

            // If the calculated sum in heap doesn't match current reality, skip (lazy deletion)
            if (nums[i] as i64 + nums[j] as i64) != sum {
                continue;
            }

            // --- PERFORM MERGE ---
            ops += 1;

            // Update Inversion Count: Remove old relationships involving i and j
            // Check relation (prev -> i)
            let prev = left[i];
            if prev != usize::MAX && nums[prev] > nums[i] {
                unsorted_count -= 1;
            }
            // Check relation (i -> j)
            if nums[i] > nums[j] {
                unsorted_count -= 1;
            }
            // Check relation (j -> next)
            let next = right[j];
            if next < n && nums[j] > nums[next] {
                unsorted_count -= 1;
            }

            // Update value of i (merge j into i)
            nums[i] = sum as i32; // Assuming the sum fits in i32 as per original logic, otherwise use i64
            valid[j] = false;     // j is effectively removed

            // Update Pointers (Remove j from linked list)
            right[i] = next;
            if next < n {
                left[next] = i;
            }

            // Update Inversion Count: Add new relationships involving new i
            // Check relation (prev -> new_i)
            if prev != usize::MAX && nums[prev] > nums[i] {
                unsorted_count += 1;
            }
            // Check relation (new_i -> next)
            if next < n && nums[i] > nums[next] {
                unsorted_count += 1;
            }

            // Check if sorted
            if unsorted_count == 0 {
                return ops;
            }

            // Add new potential merges to Heap
            // 1. Merge (prev, new_i)
            if prev != usize::MAX {
                heap.push(Reverse((nums[prev] as i64 + nums[i] as i64, prev)));
            }
            // 2. Merge (new_i, next)
            if next < n {
                heap.push(Reverse((nums[i] as i64 + nums[next] as i64, i)));
            }
        }

        ops
    }
}
