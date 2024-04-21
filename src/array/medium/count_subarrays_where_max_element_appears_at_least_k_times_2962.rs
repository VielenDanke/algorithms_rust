pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
    let (mut left, mut max_num_counter, mut result, max_num) = (0usize, 0i32, 0usize, nums.iter().max().unwrap());

    for num in &nums {
        if num == max_num {
            max_num_counter += 1;
        }
        while max_num_counter == k {
            if &nums[left] == max_num {
                max_num_counter -= 1;
            }
            left += 1;
        }
        result += left;
    }
    result as i64
}
