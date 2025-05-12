pub struct Solution;

impl Solution {
    pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        fn calc_sum_with_zero(nums: &Vec<i32>) -> (i64, i32) {
            let mut zeroes = 0; 
            let total = nums
                .iter()
                .map(|&v| v as i64)
                .map(|v| if v == 0 { zeroes += 1; 1 } else { v })
                .sum::<i64>();
            (total, zeroes)
        }
        let (total_left, zero_left) = calc_sum_with_zero(&nums1);
        let (total_right, zero_right) = calc_sum_with_zero(&nums2);

        if (zero_left == 0 && total_right > total_left)
            || (zero_right == 0 && total_left > total_right)
        {
            -1
        } else {
            total_left.max(total_right)
        }
    }
}
