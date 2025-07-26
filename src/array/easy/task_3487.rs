use std::collections::{BTreeSet, HashSet};

pub struct Solution;

impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = HashSet::from_iter(nums);
        let sum = set.iter().filter(|&x| x > &0).sum();

        if sum == 0 {
            *set.iter().max().unwrap_or(&0)
        } else {
            sum
        }
    }

    pub fn max_sum_sorting(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut b_tree_set = BTreeSet::from_iter(nums);
        
        if b_tree_set.last().unwrap() < &0 {
            *b_tree_set.last().unwrap()
        } else {
            let mut sum = 0;
            
            while let Some(x) = b_tree_set.pop_last() {
                if x < 0 {
                    break
                }
                sum += x;
            }
            sum
        }
    }
}
