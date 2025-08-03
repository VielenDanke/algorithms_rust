pub struct Solution;

impl Solution {
    pub fn count_max_or_subsets_shorter(nums: Vec<i32>) -> i32 {
        fn backtrack(nums: &[i32], max_or: i32, pos: usize, curr_or: i32) -> i32 {
            if curr_or == max_or {
                return 1 << nums.len() - pos;
            }

            if pos == nums.len() {
                return 0;
            }

            backtrack(nums, max_or, pos + 1, curr_or | nums[pos])
                + backtrack(nums, max_or, pos + 1, curr_or)
        }
        backtrack(&nums, nums.iter().fold(0, |acc, &num| acc | num), 0, 0)
    }

    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        fn subsets(result: &mut Vec<Vec<i32>>, temp: &mut Vec<i32>, nums: &Vec<i32>, start: usize) {
            if !temp.is_empty() {
                result.push(temp.clone());
            }
            for i in start..nums.len() {
                temp.push(nums[i]);
                subsets(result, temp, nums, i + 1);
                temp.pop();
            }
        }
        let mut result = vec![];

        subsets(&mut result, &mut vec![], &nums, 0);

        let mut max_val = 0;
        let mut max_counter = 0;

        for res in result {
            let mut current_val = res[0];

            for i in 1..res.len() {
                current_val |= res[i];
            }
            if max_val < current_val {
                max_val = current_val;
                max_counter = 1;
            } else if max_val == current_val {
                max_counter += 1;
            }
        }
        max_counter
    }
}
