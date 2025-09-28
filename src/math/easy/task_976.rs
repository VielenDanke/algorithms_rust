pub struct Solution;

impl Solution {
    pub fn largest_perimeter_brute_force(nums: Vec<i32>) -> i32 {
        let mut largest_perimeter = 0;

        let n = nums.len();

        for i in 0..n {
            for j in i+1..n {
                for k in j+1..n {
                    if (nums[i] + nums[j] > nums[k]) && (nums[j] + nums[k] > nums[i]) && (nums[k] + nums[i] > nums[j]) {
                        largest_perimeter = largest_perimeter.max(nums[i] + nums[j] + nums[k]);
                    }
                }
            }
        }
        largest_perimeter
    }

    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        for triplet in nums.windows(3).rev() {
            let a = triplet[0];
            let b = triplet[1];
            let c = triplet[2];

            if a + b > c {
                return a + b + c;
            }
        }
        0
    }
}