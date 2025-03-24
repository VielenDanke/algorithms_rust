pub struct Solution;

impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut window = n;
        let mut longest = 0;

        -1
    }
    
    pub fn longest_nice_subarray_brute_force(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut window = n;
        let mut longest = 0;

        while window > 0 {
            let mut idx = 0;

            while idx + window <= n {
                let mut is_valid = true;

                'teg: for i in idx..idx + window {
                    for j in i + 1..idx + window {
                        if nums[i] & nums[j] != 0 {
                            is_valid = false;
                            break 'teg;
                        }
                    }
                }
                idx += 1;

                if is_valid {
                    longest = longest.max(window);
                }
            }
            window -= 1;
        }
        longest as i32
    }
}
