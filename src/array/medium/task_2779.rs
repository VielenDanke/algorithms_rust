pub struct Solution;

impl Solution {
    pub fn maximum_beauty(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();

        let (mut i, mut j, n) = (0, 0, nums.len());

        while j < n {
            if nums[j] - nums[i] > k * 2 {
                i += 1;
            }
            j += 1;
        }
        (j - i) as i32
    }

    pub fn maximum_beauty_brute_force(mut nums: Vec<i32>, k: i32) -> i32 {
        if nums.len() == 1 {
            return 1;
        }

        let (min, max, mut result) = (*nums.iter().min().unwrap(), *nums.iter().max().unwrap(), 0);

        if max - min <= 2 * k {
            return nums.len() as i32;
        }

        for v in min + k..max - k + 1 {
            let mut count = 0;

            for &num in nums.iter() {
                if v - k <= num && num <= v + k {
                    count += 1;
                }
            }
            result = result.max(count);
        }
        result
    }
}