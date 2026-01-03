use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn repeated_n_times_sorting(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        for i in 0..nums.len() - 1 {
            if nums[i] == nums[i + 1] {
                return nums[i];
            }
        }

        -1
    }

    pub fn repeated_n_times_hash_set(nums: Vec<i32>) -> i32 {
        let mut visited = HashSet::new();

        for num in nums {
            if !visited.insert(num) {
                return num;
            }
        }

        -1
    }

    pub fn repeated_n_times_array(nums: Vec<i32>) -> i32 {
        let mut visited = vec![false; 10001];

        for num in nums {
            if visited[num as usize] {
                return num;
            }
            visited[num as usize] = true;
        }

        -1
    }

    pub fn repeated_n_times_bit_manipulation(nums: Vec<i32>) -> i32 {
        let len = nums.len();

        // Check distances k = 1, 2, and 3.
        // Due to the density of the repeated element (n times in 2n length),
        // two instances are guaranteed to be at most 3 indices apart.
        for k in 1..=3 {
            for i in 0..(len - k) {
                if (nums[i] ^ nums[i + k]) == 0 {
                    return nums[i];
                }
            }
        }

        // This part is theoretically unreachable given the problem constraints
        // unless n=1, in which case the loop above works, or the input is invalid.
        nums[0]
    }
}
