use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut freq = HashMap::new();

        let mut max_freq = 0;

        for &num in nums.iter() {
            let elem = freq.entry(num).or_insert(0);
            *elem += 1;
            if *elem > max_freq {
                max_freq = *elem;
            }
        }
        let mut counter = 0;

        for num in nums {
            if let Some(&val) = freq.get(&num) {
                if val == max_freq {
                    counter += 1;
                }
            }
        }
        counter
    }

    pub fn max_frequency_elements_vec(nums: Vec<i32>) -> i32 {
        let mut freq = vec![0; 101];

        let mut max_freq = 0;

        for &num in nums.iter() {
            freq[num as usize] += 1;
            if freq[num as usize] > max_freq {
                max_freq = freq[num as usize];
            }
        }
        let mut counter = 0;

        for num in nums {
            if freq[num as usize] == max_freq {
                counter += 1;
            }
        }
        counter
    }
}
