use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut bad_pairs = 0;
        let mut counter = HashMap::new();

        for i in 0..n {
            // nums[i], i
            // nums[j]?, j?
            // x - i = y - nums[i], x - y = i - nums[i]
            let diff = i as i64 - nums[i] as i64;
            
            let good_pairs = *counter.entry(diff).or_insert(0);
            
            bad_pairs += i as i64 - good_pairs;
            
            counter.insert(diff, good_pairs + 1);
        }

        bad_pairs
    }
    
    pub fn count_bad_pairs_tle(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut bad_pairs = 0;
        
        for i in 0..n {
            for j in (i + 1)..n {
                if (j - i) as i32 != nums[j] - nums[i] {
                    bad_pairs += 1;
                }
            }
        }
        
        bad_pairs
    }
}