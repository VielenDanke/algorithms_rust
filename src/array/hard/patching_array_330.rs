pub struct Solution;

impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let (mut index, mut count, mut value_need) = (0usize, 0, 1i64);

        while value_need <= n as i64 {
            if index < nums.len() {
                let current = nums[index] as i64;
                if value_need >= current {
                    value_need += current;
                    index += 1;
                    continue;
                }
            }
            value_need += value_need;
            count += 1;
        }

        count
    }
}
