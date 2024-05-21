pub struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        Self::backtrack(&nums, &mut Vec::new(), &mut result, 0);
        result
    }

    fn backtrack(nums: &Vec<i32>, temp: &mut Vec<i32>, result: &mut Vec<Vec<i32>>, start: usize) {
        result.push(temp.clone());
        for i in start..nums.len() {
            temp.push(*nums.get(i).unwrap());
            Self::backtrack(nums, temp, result, i + 1);
            temp.remove(temp.len() - 1);
        }
    }
}
