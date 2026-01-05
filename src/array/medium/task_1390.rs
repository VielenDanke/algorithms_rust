use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn sum_four_divisors_math(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        for num in nums {
            let mut current_result = 0;
            let mut counter = 0;
            for divisor in 1..=num.isqrt() {
                if num % divisor == 0 {
                    if num / divisor == divisor {
                        current_result += divisor;
                        counter += 1;
                    } else {
                        current_result += num / divisor;
                        current_result += divisor;
                        counter += 2;
                    }

                    if counter > 4 {
                        break;
                    }
                }
            }

            if counter == 4 {
                result += current_result;
            }
        }

        result
    }

    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        let mut total_sum = 0;

        for &n in &nums {
            // Numbers less than 6 (1, 2, 3, 4, 5) cannot have 4 divisors.
            // 4 is a square (3 divisors), the rest have 1 or 2.
            if n < 6 {
                continue;
            }

            let sqrt = (n as f64).sqrt() as i32;

            // Optimization: Perfect squares have an odd number of divisors.
            // They can never have exactly 4.
            if sqrt * sqrt == n {
                continue;
            }

            // Start with divisors 1 and n
            let mut sum = 1 + n;
            let mut count = 2;

            for i in 2..=sqrt {
                if n % i == 0 {
                    count += 2;
                    sum += i + (n / i);

                    // If we exceed 4 divisors, stop immediately
                    if count > 4 {
                        break;
                    }
                }
            }

            if count == 4 {
                total_sum += sum;
            }
        }

        total_sum
    }

    pub fn sum_four_divisors_cache_brute_force(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut cache = HashMap::new();

        for num in nums {
            if let Some(v) = cache.get(&num) {
                result += v;
            } else {
                let mut count = 0;
                let mut temp = 0;

                for i in 1..=num {
                    if num % i == 0 {
                        count += 1;
                        temp += i;

                        if count > 4 {
                            break;
                        }
                    }
                }

                if count == 4 {
                    result += temp;
                    cache.insert(num, temp);
                } else {
                    cache.insert(num, 0);
                }
            }
        }

        result
    }
}
