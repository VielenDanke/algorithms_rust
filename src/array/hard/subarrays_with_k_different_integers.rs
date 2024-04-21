use std::collections::HashMap;

fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
    sliding_window_at_most(&nums, k) - sliding_window_at_most(&nums, k - 1)
}

fn sliding_window_at_most(nums: &Vec<i32>, distinct_k: i32) -> i32 {
    let mut freq_map: HashMap<i32, i32> = HashMap::new();
    let (mut left, mut total_count) = (0, 0);

    for (right, &num) in nums.iter().enumerate() {
        *freq_map.entry(num).or_insert(0) += 1;

        while freq_map.len() > distinct_k as usize {
            if let Some(entry) = freq_map.get_mut(&nums[left]) {
                *entry -= 1;
                if *entry == 0 {
                    freq_map.remove(&nums[left]);
                }
            }
            left += 1;
        }
        total_count += (right - left + 1) as i32;
    }
    total_count
}
