pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let (mut start, mut end) = (0 as usize, nums.len() - 1);
    let mut output = vec![0i32; nums.len()];

    for i in (0..output.len()).rev() {
        let (start_num, end_num) = (nums[start] * nums[start], nums[end] * nums[end]);

        if start_num > end_num {
            output[i] = start_num;
            start += 1;
        } else {
            output[i] = end_num;
            end -= 1;
        }
    }
    output
}

pub fn sorted_squares_sort(nums: Vec<i32>) -> Vec<i32> {
    let mut output: Vec<i32> = nums.iter()
        .map(|num| num * num)
        .collect();
    output.sort();
    output
}
