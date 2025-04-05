pub struct Solution;

impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let mut max = 0;
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                for k in j + 1..nums.len() {
                    max = max.max((nums[i] as i64 - nums[j] as i64) * nums[k] as i64);
                }
            }
        }
        max
    }

    pub fn maximum_triplet_value_sort(mut nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut result = 0;
        
        for i in 2..n {
            let mut max_prefix = nums[0];
            for j in 1..i {
                result = result.max((max_prefix - nums[j]) as i64 * nums[i] as i64);
                max_prefix = max_prefix.max(nums[j]);
            }
        }
        result
    }
}
