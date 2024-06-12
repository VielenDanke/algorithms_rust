pub struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        nums.sort_unstable_by(|left, right| left.cmp(right));
    }

    pub fn sort_colors_more(nums: &mut Vec<i32>) {
        for i in 0..nums.len() {
            let mut j = i;

            while j > 0 {
                if nums[j - 1] > nums[j] {
                    let temp = nums[j - 1];
                    nums[j - 1] = nums[j];
                    nums[j] = temp;
                }
                j -= 1;
            }
        }
    }

    pub fn sort_colors_count(nums: &mut Vec<i32>) {
        let mut count_sort = vec![0; 3];

        for &num in nums.iter() {
            count_sort[num as usize] += 1;
        }
        let mut idx = 0;

        for i in 0..count_sort.len() {
            let mut temp = count_sort[i];

            while temp > 0 {
                nums[idx] = i as i32;
                idx += 1;
                temp -= 1;
            }
        }
    }

    pub fn sort_colors_bubble_sort(nums: &mut Vec<i32>) {
        let mut is_sorted = false;

        while !is_sorted {
            is_sorted = true;

            for i in 1..nums.len() {
                if nums[i - 1] > nums[i] {
                    let temp = nums[i];
                    nums[i] = nums[i - 1];
                    nums[i - 1] = temp;
                    is_sorted = false;
                }
            }
        }
    }
}
