pub struct Solution;

impl Solution {
    pub fn minimum_pair_removal(mut nums: Vec<i32>) -> i32 {
        let mut counter = 0;
        while !nums.is_empty() {
            if nums.is_sorted() {
               return counter
            }
            let mut min_sum = 1 << 30;
            for i in 0..nums.len() - 1 {
                min_sum = min_sum.min(nums[i] + nums[i + 1]);
            }
            for i in 0..nums.len() - 1 {
                if min_sum == nums[i] + nums[i+1] {
                    counter += 1;
                    nums[i] = min_sum;
                    nums.remove(i + 1);
                    break;
                }
            }
        }
        0
    }
}