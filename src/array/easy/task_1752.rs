pub struct Solution;

impl Solution {
    pub fn check_find_smallest(nums: Vec<i32>) -> bool {
        let n = nums.len();

        if n <= 1 {
            return true;
        }
        let mut inversion_count = 0;

        for i in 1..n {
            if nums[i] < nums[i - 1] {
                inversion_count += 1;
            }
        }
        if nums[0] < nums[n - 1] {
            inversion_count += 1;
        }
        inversion_count <= 1
    }

    pub fn check(nums: Vec<i32>) -> bool {
        let min_number = *nums.iter().min().unwrap();

        for i in 0..nums.len() {
            let mut is_sorted = false;

            if nums[i] == min_number {
                is_sorted = true;

                let mut idx = i;

                while (idx + 1) % nums.len() != i {
                    idx = idx % nums.len();
                    let next_idx = (idx + 1) % nums.len();

                    if nums[next_idx] < nums[idx] {
                        is_sorted = false;
                        break;
                    }
                    idx += 1;
                }
            }
            if is_sorted {
                return true;
            }
        }
        false
    }
}
