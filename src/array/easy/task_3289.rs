pub struct Solution;

impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut counter = vec![0; nums.len()];
        let mut result = vec![];

        for &num in nums.iter() {
            counter[num as usize] += 1;
            if counter[num as usize] > 1 {
                if result.is_empty() || result[0] != num {
                    result.push(num);
                }
                if result.len() == 2 {
                    return result
                }
            }
        }
        result
    }
}