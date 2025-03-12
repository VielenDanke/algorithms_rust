pub struct Solution;

impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let negative_idx = nums.partition_point(|n| *n < 0);
        let mut positive_idx = negative_idx;
        while positive_idx < n && nums[positive_idx] == 0 && negative_idx < n - positive_idx {
            positive_idx += 1;
        }
        negative_idx.max(n - positive_idx) as i32
    }

    pub fn maximum_count_shorter(nums: Vec<i32>) -> i32 {
        nums.partition_point(|&num| num < 0)
            .max(nums.len() - nums.partition_point(|&num| num <= 0)) as i32
    }
}
