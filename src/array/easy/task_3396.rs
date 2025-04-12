pub struct Solution;

impl Solution {
    pub fn minimum_operations(mut nums: Vec<i32>) -> i32 {
        fn is_distinct(nums: &Vec<i32>) -> bool {
            nums.iter().all(|num| num <= &1)
        }
        let mut elem_counter = vec![0; 101];
        
        for &num in nums.iter() {
            elem_counter[num as usize] += 1;
        }
        let mut start_idx = 0;
        let mut counter = 0;
        
        while start_idx < nums.len() {
            if is_distinct(&elem_counter) {
                return counter;
            }
            for i in start_idx..(start_idx + 3).min(nums.len()) {
                elem_counter[nums[i] as usize] -= 1;
            }
            counter += 1;
            start_idx = start_idx + 3;
        }
        if is_distinct(&elem_counter) {
            counter
        } else {
            0
        }
    }
}