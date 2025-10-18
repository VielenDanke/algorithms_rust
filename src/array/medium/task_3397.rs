pub struct Solution;

/*
You are given an integer array nums and an integer k.

You are allowed to perform the following operation on each element of the array at most once:

Add an integer in the range [-k, k] to the element.
Return the maximum possible number of distinct elements in nums after performing the operations.
 */

impl Solution {
    pub fn max_distinct_elements(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let mut prev_assigned: i64 = -1 << 30;
        let mut distinct_count: i32 = 0;
        let k = k as i64;
        for num in nums.iter().map(|&v| v as i64) {
            let assigned = std::cmp::max(num - k, prev_assigned + 1);
            if assigned <= num + k {
                distinct_count += 1;
                prev_assigned = assigned;
            }
        }
        distinct_count
    }
}
