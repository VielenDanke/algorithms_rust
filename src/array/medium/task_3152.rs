pub struct Solution;

impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut result = Vec::new();

        for query in queries.iter() {
            let (mut left, mut right) = (query[0] as usize, query[1] as usize);

            let mut current_result = true;

            for i in left..right {
                if (nums[i] % 2 == 0 && nums[i + 1] % 2 == 0) || (nums[i] % 2 == 1 && nums[i + 1] % 2 == 1) {
                    current_result = false;
                    break;
                }
            }
            result.push(current_result);
        }
        result
    }
}