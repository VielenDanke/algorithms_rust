pub struct Solution;

/*
You are given an integer array nums of size n where n is a multiple of 3 and a positive integer k.

Divide the array nums into n / 3 arrays of size 3 satisfying the following condition:

The difference between any two elements in one array is less than or equal to k.
Return a 2D array containing the arrays. If it is impossible to satisfy the conditions,
return an empty array. And if there are multiple answers, return any of them.
 */

impl Solution {
    pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        let mut result = Vec::new();

        let n = nums.len();
        if n % 3 != 0 {
            return vec![];
        }

        for i in (0..n).step_by(3) {
            if i + 2 >= n || nums[i + 2] - nums[i] > k {
                return vec![];
            }
            result.push(vec![nums[i], nums[i + 1], nums[i + 2]]);
        }

        result
    }
}