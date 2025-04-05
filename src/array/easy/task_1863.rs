pub struct Solution;

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        fn generate_all_subsets(nums: &Vec<i32>, idx: usize, result: &mut Vec<Vec<i32>>, temp: &mut Vec<i32>) {
            result.push(temp.clone());
            for i in idx..nums.len() {
                temp.push(nums[i]);
                generate_all_subsets(nums, i + 1, result, temp);
                temp.pop();
            }
        }
        let mut total_subsets = vec![];
        
        generate_all_subsets(&nums, 0,  &mut total_subsets, &mut vec![]);
        
        let mut result = 0;
        
        for subset in total_subsets {
            let mut subset_xor = 0;
            
            for &elem in subset.iter() {
                subset_xor ^= elem;
            }
            result += subset_xor;
        }
        result
    }

    pub fn subset_xor_sum_faster(nums: Vec<i32>) -> i32 {
        fn xor_sum(nums: &Vec<i32>, idx: usize, current_xor: i32) -> i32 {
            if idx == nums.len() {
                current_xor
            } else {
                let with_element = xor_sum(nums, idx + 1, current_xor ^ nums[idx]);
                let without_element = xor_sum(nums, idx + 1, current_xor);
                with_element + without_element
            }
        }
        xor_sum(&nums, 0, 0)
    }
}