pub struct Solution;

impl Solution {
    pub fn num_subseq_easier(mut nums: Vec<i32>, target: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        nums.sort_unstable();
        let n = nums.len();

        // Precompute powers of 2
        let mut power = vec![1; n];
        for i in 1..n {
            power[i] = (power[i - 1] * 2) % MOD;
        }

        let (mut left, mut right) = (0, n - 1);
        let mut res = 0;

        while left <= right {
            if nums[left] + nums[right] <= target {
                res = (res + power[right - left]) % MOD;
                left += 1;
            } else {
                if right == 0 {
                    break;
                }
                right -= 1;
            }
        }

        res
    }

    fn mod_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
        let mut result = 1;
        base %= modulus;

        while exp > 0 {
            if exp % 2 == 1 {
                result = result * base % modulus;
            }
            base = base * base % modulus;
            exp /= 2;
        }

        result
    }

    pub fn num_subseq(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();

        let boundary = match nums.iter().position(|&x| x > target) {
            Some(i) => i,
            None => nums.len(),
        };

        const MOD: u128 = 1_000_000_007;
        let mut subs: u128 = 0;
        let mut l = 0;
        let mut r = boundary - 1;

        if r == 0 {
            return 0;
        }

        while l <= r {
            let min_v = nums[l];
            let max_v = nums[r];

            if min_v + max_v > target {
                r -= 1;
            } else {
                let len = r - l;
                let x = Self::mod_pow(2, len as u128, MOD);
                subs += x;
                l += 1;
            }
        }

        (subs % MOD) as i32
    }

    pub fn num_subseq_brute_force(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();

        let total = 1usize << nums.len();
        let mut counter = 0;

        for mask in 0..total {
            // Build a subsequence whose membership is determined by the bitmask
            let subseq: Vec<i32> = nums
                .iter()
                .enumerate()
                .filter_map(|(idx, item)| {
                    if mask & (1 << idx) != 0 {
                        Some(item.clone())
                    } else {
                        None
                    }
                })
                .collect();
            if !subseq.is_empty() && subseq.first().unwrap() + subseq.last().unwrap() <= target {
                counter += 1;
            }
        }
        counter
    }
}
