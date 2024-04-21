pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
    let mut array = vec![0i32; 101];

    for num in nums.iter() {
        array[*num as usize] += 1;
    }
    let mut max_freq = -1 << 30;
    let mut elements = 0;

    for num in nums.iter() {
        let current_freq = array[*num as usize];
        if current_freq > 0 && current_freq > max_freq {
            max_freq = current_freq;
            elements = 1;
        } else if current_freq == max_freq {
            elements += 1;
        }
    }
    elements
}
