pub struct Solution;

impl Solution {
    pub fn minimum_cost(nums: Vec<i32>) -> i32 {
        // [10,6,5,3,1,1]
        // 3 arrays
        let mut arr = vec![51, 51, nums[0]];

        for i in 1..nums.len() {
            if nums[i] < arr[0] {
                arr[1] = arr[0];
                arr[0] = nums[i];
            } else if nums[i] < arr[1] {
                arr[1] = nums[i];
            }
            if arr[0] == 1 && arr[1] == 1 {
                break;
            }
        }
        arr.iter().sum::<i32>()
    }
}
