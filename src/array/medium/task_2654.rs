pub struct Solution;

impl Solution {
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let temp = a;
            a = b;
            b = temp % b;
        }
        a
    }

    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let ones = nums.iter().filter(|&&n| n == 1).count() as i32;

        let n = nums.len();

        if ones > 0 {
            n as i32 - ones
        } else {
            let mut min_len = n + 1;

            for i in 0..n {
                let mut current = nums[i];

                for j in i + 1..n {
                    current = Self::gcd(current, nums[j]);

                    if current == 1 {
                        let m = j - i + 1;

                        min_len = min_len.min(m);

                        break;
                    }
                }
            }
            if min_len == n + 1 {
                -1
            } else {
                ((min_len - 1) + (n - 1)) as i32
            }
        }
    }
}