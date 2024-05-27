pub struct Solution;

impl Solution {
    pub fn special_array(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();

        nums.sort_unstable();

        if nums[0] >= n as i32 {
            n as i32
        } else {
            let mut result = -1;

            for i in 1..n {
                let diff = (n - i) as i32;

                if nums[i] >= diff && nums[i - 1] < diff {
                    result = diff;
                    break;
                }
            }
            result
        }
    }
}
