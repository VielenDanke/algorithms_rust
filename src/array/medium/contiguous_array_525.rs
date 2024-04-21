use std::collections::HashMap;

pub fn find_max_length_shorter(nums: Vec<i32>) -> i32 {
    let (mut map, mut curr, mut ans) = (HashMap::new(), 0, 0);
    map.insert(0, -1);
    for (i, num) in nums.iter().enumerate() {
        curr += if num == &0 { -1 } else { 1 };
        if let Some(count) = map.get(&curr) {
            ans = ans.max(i as i32 - count);
        } else {
            map.insert(curr, i as i32);
        }
    }
    ans
}

pub fn find_max_length(nums: Vec<i32>) -> i32 {
    let mut count: i32 = 0;
    let mut max_length: i32 = 0;
    let mut map = HashMap::new();

    for i in 0..nums.len() {
        if nums[i] == 0 {
            count -= 1;
        } else {
            count += 1;
        }
        if map.contains_key(&count) {
            max_length = std::cmp::max(max_length, (i - map.get(&count).unwrap()) as i32);
        } else if count == 0 {
            max_length = std::cmp::max(max_length, i as i32 + 1);
        } else {
            map.insert(count, i);
        }
    }
    return max_length;
}

pub fn find_max_length_brute_force(nums: Vec<i32>) -> i32 {
    let (mut right, mut max) = (1, 0);

    while right <= nums.len() {
        let mut left = 0;
        while left + right <= nums.len() {
            let slice = &nums[left..(left + right)];
            let (mut zeroes, mut ones) = (0, 0);

            for num in slice {
                if *num == 0 {
                    zeroes += 1;
                } else {
                    ones += 1;
                }
            }
            if zeroes == ones && right > max {
                max = right;
            }
            left += 1;
        }
        right += 1
    }
    max as i32
}
