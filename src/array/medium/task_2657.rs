use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let n = a.len();
        let mut result = vec![0; n];
        let mut a_set = HashSet::new();
        let mut b_set = HashSet::new();
        
        for i in 0..n {
            a_set.insert(a[i]);
            b_set.insert(b[i]);
            result[i] = a_set.intersection(&b_set).count() as i32;
        }
        result
    }

    pub fn find_the_prefix_common_array_frequency_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let n = a.len();
        let mut result = vec![0; n];
        let mut freq = vec![0; n + 1];
        let mut common_count = 0;
        
        for i in 0..n {
            freq[a[i] as usize] += 1;
            
            if freq[a[i] as usize] == 2 {
                common_count += 1;
            }
            
            freq[b[i] as usize] += 1;
            
            if freq[b[i] as usize] == 2 {
                common_count += 1;
            }
            result[i] = common_count;
        }
        result
    }
}