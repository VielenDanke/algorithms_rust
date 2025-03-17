pub struct Solution;

impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let mut counter = vec![0; 501];
        
        for &num in nums.iter() {
            counter[num as usize] += 1;
        }
        counter.iter().all(|&x| x % 2 == 0)
    }
}