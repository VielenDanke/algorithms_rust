pub struct Solution;

impl Solution {
    pub fn count_symmetric_integers_faster(low: i32, high: i32) -> i32 {
        let mut result = 0;
        
        for num in low..=high {
            if num < 100 && num % 11 == 0 {
                result += 1;
            } else if 1000 <= num && num < 10000 {
                let left = num / 1000 + (num % 1000) / 100;
                let right = (num % 100) / 10 + num % 10;
                if left == right {
                    result += 1;
                }
            }
        }
        result
    }
    
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        let mut counter = 0;

        for mut num in low..=high {
            let mut store = vec![];
            
            while num > 0 {
                store.push(num % 10);
                num /= 10;
            }
            let n = store.len();

            if n == 0 || n == 1 {
                continue;
            }
            let mut left_sum: i32 = 0;
            let mut right_sum: i32 = 0;

            if n % 2 != 0 {
                left_sum = store[0..(n / 2 - 1)].iter().sum::<i32>() ;
                right_sum = store[(n / 2 + 1)..n].iter().sum::<i32>();
            } else {
                left_sum = store[0..n / 2].iter().sum::<i32>();
                right_sum = store[n / 2..n].iter().sum::<i32>();
            }
            if left_sum == right_sum {
                counter += 1;
            }
        }
        counter
    }
}
