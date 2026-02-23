pub struct Solution;

impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        let mut result = 0;

        fn is_prime(n: u32) -> bool {
            if n <= 1 {
                false
            } else {
                let mut num = 2;

                while num * num <= n {
                    if n % num == 0 {
                        return false;
                    }
                    num += 1;
                }
                true
            }
        }

        for num in left..=right {
            let ones = num.count_ones();
            if is_prime(ones) {
                result += 1;
            }
        }
        result
    }
}