use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let mut counter = HashMap::new();
        
        for num in digits {
            *counter.entry(num).or_insert(0) += 1;
        }
        let mut result = vec![];
        
        for mut digit in 100..999 {
            if digit % 2 == 0 {
                let to_insert = digit;
                let mut is_correct = true;
                let mut cloned_counter = counter.clone();
                
                while digit > 0 {
                    let last_digit = digit % 10;

                    let counter_num = cloned_counter.entry(last_digit).or_insert(0);
                    
                    if *counter_num <= 0 {
                        is_correct = false;
                        break
                    }
                    *counter_num -= 1;
                    
                    digit /= 10;
                }
                if is_correct {
                    result.push(to_insert);
                }
            }
        } 
        result
    }
}