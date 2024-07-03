pub struct Solution;

impl Solution {

    pub fn min_difference_sort(mut nums: Vec<i32>) -> i32 {
        if nums.len() < 5 {
            0
        } else {
            nums.sort_unstable();

            (nums[nums.len() - 1] - nums[3])
                .min(nums[nums.len() - 2] - nums[2])
                .min(nums[nums.len() - 3] - nums[1])
                .min(nums[nums.len() - 4] - nums[0])
        }
    }

    pub fn min_difference(nums: Vec<i32>) -> i32 {
        if nums.len() <= 4 {
            // Can change all values to be the same
            return 0;
        }
        let mut four_smallest = [i32::MAX; 4];
        let mut four_largest = [i32::MIN; 4];

        for num in nums {
            if num <= four_smallest[0] {
                four_smallest = [num, four_smallest[0], four_smallest[1], four_smallest[2]];
            } else if num <= four_smallest[1] {
                four_smallest = [four_smallest[0], num, four_smallest[1], four_smallest[2]];
            } else if num <= four_smallest[2] {
                four_smallest = [four_smallest[0], four_smallest[1], num, four_smallest[2]];
            } else if num <= four_smallest[3] {
                four_smallest = [four_smallest[0], four_smallest[1], four_smallest[2], num];
            }

            if num >= four_largest[3] {
                four_largest = [four_largest[1], four_largest[2], four_largest[3], num];
            } else if num >= four_largest[2] {
                four_largest = [four_largest[1], four_largest[2], num, four_largest[3]];
            } else if num >= four_largest[1] {
                four_largest = [four_largest[1], num, four_largest[2], four_largest[3]];
            } else if num >= four_largest[0] {
                four_largest = [num, four_largest[1], four_largest[2], four_largest[3]];
            }
        }
        four_largest.into_iter().zip(four_smallest.into_iter()).map(|(big, little)| big - little).min().unwrap()
    }
}
