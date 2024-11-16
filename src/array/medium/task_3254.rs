pub struct Solution;

impl Solution {
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut idx = 0usize;
        let n = nums.len();
        let k = k as usize;

        let mut result = Vec::new();

        'outer: while idx + k <= n {
            let slice = &nums[idx..idx + k];

            idx += 1;

            let last_element = slice[k - 1];

            if (last_element - slice[0]) + 1 != k as i32 {
                result.push(-1);
                continue
            }
            let mut prev = None;

            for &elem in slice.iter() {
                if prev.is_some() && elem - prev.unwrap() != 1 {
                    result.push(-1);
                    continue 'outer;
                }
                prev = Some(elem);
            }
            result.push(last_element);
        }
        result
    }
}