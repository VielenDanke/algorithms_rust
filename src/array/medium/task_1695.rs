/*
You are given an array of positive integers nums and want to erase a subarray containing unique elements.
The score you get by erasing the subarray is equal to the sum of its elements.

Return the maximum score you can get by erasing exactly one subarray.

An array b is called to be a subarray of a if it forms a contiguous subsequence of a, that is, if it is equal to a[l],a[l+1],...,a[r]
for some (l,r).
 */
use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = 0;
        let mut current_sum = 0;
        let mut max = 0;

        let mut visited = HashSet::new();

        while right < nums.len() {
            if visited.insert(nums[right]) {
                current_sum += nums[right];
                max = max.max(current_sum);
                right += 1;
            } else {
                current_sum -= nums[left];
                visited.remove(&nums[left]);
                left += 1;
            }
        }
        max
    }
}
