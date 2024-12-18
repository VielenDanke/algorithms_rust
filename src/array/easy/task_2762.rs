pub struct Solution;

impl Solution {
    pub fn continuous_subarrays(nums: Vec<i32>) -> i64 {
        let mut lptr = nums[0] - 2;
        let mut rptr = nums[0] + 2;
        let mut j = 0;
        let mut ans = 1;
        for i in 1..nums.len() {
            if nums[i] >= lptr && nums[i] <= rptr {
                lptr = lptr.max(nums[i] - 2);
                rptr = rptr.min(nums[i] + 2);
                ans += (i - j + 1) as i64;
            } else {
                j = i - 1;
                lptr = nums[i] - 2;
                rptr = nums[i] + 2;
                while nums[j] >= lptr && nums[j] <= rptr {
                    lptr = lptr.asmax(nums[j] - 2);
                    rptr = rptr.min(nums[j] + 2);
                    j -= 1;
                }
                ans += (i - j) as i64;
                j += 1;
            }
        }
        ans
    }

    pub fn continuous_subarrays_tle(nums: Vec<i32>) -> i64 {
        let mut window = 1;
        let n = nums.len();
        let mut result = 0;

        while window <= n {
            for i in 0..n {
                if i + window <= n {
                    let sub_array = &nums[i..i+window];
                    if sub_array.iter().max().unwrap() - sub_array.iter().min().unwrap() <= 2 {
                        result += 1;
                    }
                } else {
                    break;
                }
            }
            window += 1;
        }
        result
    }
}