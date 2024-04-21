use std::collections::HashMap;

pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    let mut map = HashMap::new();

    let (mut left, mut right, mut result) = (0usize, 0usize, 0i32);

    while right < nums.len() {
        let val = map.entry(nums[right]).or_insert(0);
        *val += 1;

        let mut val = *val;

        while val > k {
            let left_k = map.get_mut(&nums[left]).unwrap();
            *left_k -= 1;

            if nums[left] == nums[right] {
                val -= 1;
            }
            left += 1;
        }
        result = result.max((right - left + 1) as i32);
        right += 1;
    }
    result
}
