pub mod easy;
pub mod medium;
pub mod hard;

fn create_prefix_sum(nums: &Vec<i32>) -> Vec<i32> {
    let mut prefix_sum = vec![0; nums.len()];

    prefix_sum[0] = nums[0];

    for i in 1..nums.len() {
        prefix_sum[i] = prefix_sum[i - 1] + nums[i];
    }
    prefix_sum
}
