pub struct Solution;

impl Solution {
    pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
        let u_value = value as usize;
        let mut multiset = vec![0; u_value];

        for &n in &nums {
            multiset[n.rem_euclid(value) as usize] += 1;
        }

        for n in 0..=nums.len() {
            if multiset[n % u_value] == 0 {
                return n as i32;
            }
            multiset[n % u_value] -= 1;
        }

        0
    }
}