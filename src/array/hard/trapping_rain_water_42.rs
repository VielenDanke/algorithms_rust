struct Solution {}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let (mut left_max, mut right_max, n) = (0, 0, height.len());

        let mut maxes = vec![0; n];

        for i in 0..n {
            let current = height[i];
            maxes[i] = left_max;
            left_max = left_max.max(current);
        }
        for i in (0..n).rev() {
            let current = height[i];
            let min_height = right_max.min(maxes[i]);
            if current < min_height {
                maxes[i] = min_height - current;
            } else {
                maxes[i] = 0;
            }
            right_max = right_max.max(current);
        }
        maxes.iter().sum()
    }

    pub fn trap_shorter(height: Vec<i32>) -> i32 {
        let (mut l_idx, mut r_idx, mut l_max, mut r_max, mut result) = (0_usize, height.len() - 1, 0, 0, 0);
        while l_idx <= r_idx {
            match height[l_idx] <= height[r_idx] {
                true if height[l_idx] >= l_max => {
                    l_max = height[l_idx];
                    l_idx += 1;
                }
                true => {
                    result += l_max - height[l_idx];
                    l_idx += 1;
                }
                false if height[r_idx] >= r_max => {
                    r_max = height[r_idx];
                    r_idx -= 1;
                }
                false => {
                    result += r_max - height[r_idx];
                    r_idx -= 1;
                }
            }
        }
        result
    }
}
