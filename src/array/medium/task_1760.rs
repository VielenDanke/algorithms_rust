pub struct Solution;

impl Solution {
    pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        let mut left  = 1;
        let mut right = *nums.iter().max().unwrap();

        while left < right {
            let mid = left + (right - left) / 2;
            let ops = nums.iter().map(|&n| (n - 1) / mid).sum::<i32>();

            if ops > max_operations {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }

    pub fn minimum_size_brute_force_second(nums: Vec<i32>, max_operations: i32) -> i32 {
        fn can_divide(nums: &Vec<i32>, max_balls: i32, max_operations: i32) -> bool {
            let mut ops = 0;
            for &n in nums {
                ops += (n as f64 / max_balls as f64).ceil() as i32 - 1;
                if ops > max_operations {
                    return false;
                }
            }
            true
        }

        for i in 1..=(*nums.iter().max().unwrap()) {
            if can_divide(&nums, i, max_operations) {
                return i;
            }
        }

        -1
    }

    pub fn minimum_size_brute_force(mut nums: Vec<i32>, max_operations: i32) -> i32 {
        Self::dfs(&mut nums, max_operations)
    }

    fn dfs(nums: &mut Vec<i32>, max_operations: i32) -> i32 {
        if max_operations == 0 {
            *nums.iter().max().unwrap()
        } else {
            let mut result = 1 << 30;
            let mut idx = 0;
            let mut max_val = 0;
            for (i, &v) in nums.iter().enumerate() {
                if v > max_val {
                    max_val = v;
                    idx = i;
                }
            }
            let removed_val = nums.remove(idx);

            if removed_val % 2 == 0 {
                nums.push(removed_val / 2);
                nums.push(removed_val / 2);
                result = result.min(Self::dfs(nums, max_operations - 1));
                nums.remove(nums.len() - 1);
                nums.remove(nums.len() - 1);
            } else {
                for v in 1..removed_val {
                    nums.push(v);
                    nums.push(removed_val - v);
                    result = result.min(Self::dfs(nums, max_operations - 1));
                    nums.remove(nums.len() - 1);
                    nums.remove(nums.len() - 1);
                }
            }
            nums.push(removed_val);
            result
        }
    }
}