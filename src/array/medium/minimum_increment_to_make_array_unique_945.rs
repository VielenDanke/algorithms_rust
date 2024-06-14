use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn min_increment_for_unique(nums: Vec<i32>) -> i32 {
        let mut counter = vec![0; 1000001];

        for &num in nums.iter() {
            counter[num as usize] += 1;
        }

        let mut increments = 0;

        for i in 0..counter.len() {
            if counter[i] > 1 {
                counter[i + 1] += counter[i] - 1;
                increments += counter[i] - 1;
            }
        }

        increments
    }

    pub fn min_increment_for_unique_sort(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let mut increments = 0;

        for i in 0..nums.len() - 1 {
            let diff = nums[i + 1] - nums[i];

            if diff <= 0 {
                nums[i + 1] += 1 - diff;
                increments += 1 - diff;
            }
        }

        increments
    }

    pub fn min_increment_for_unique_tle(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let mut increments = 0;

        let mut visited = HashSet::new();

        for i in 0..nums.len() {
            while visited.contains(&nums[i]) {
                nums[i] += 1;
                increments += 1;
            }
            visited.insert(nums[i]);
        }
        increments
    }
}
