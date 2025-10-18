pub struct Solution;

impl Solution {
    pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let window = (k * 2);

        'l: for w in nums.windows(window) {
            for i in 0..k - 1 {
                if w[i] >= w[i + 1] {
                    continue 'l;
                }
                if w[i + k] >= w[i + 1 + k] {
                    continue 'l;
                }
            }
            return true;
        }
        false
    }
}