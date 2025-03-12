use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        fn has_all_chars(freq: &[i32; 3]) -> bool {
            freq[0] > 0 && freq[1] > 0 && freq[2] > 0
        }
        
        let len = s.len();
        let mut left = 0;
        let mut right = 0;
        let mut freq = [0; 3]; 
        let mut total = 0;
        let s_bytes = s.as_bytes();

        while right < len {
            // Add character at right pointer to frequency array
            let curr = s_bytes[right] as usize - 'a' as usize;
            
            freq[curr] += 1;

            // While we have all required characters
            while has_all_chars(&freq) {
                // All substrings from current window to end are valid
                total += (len - right) as i32;

                // Remove leftmost character and move left pointer
                let left_char = s_bytes[left] as usize - 'a' as usize;
                
                freq[left_char] -= 1;
                
                left += 1;
            }

            right += 1;
        }

        total
    }
    
    pub fn number_of_substrings_brute_force(s: String) -> i32 {
        let mut window = 3;

        let s_bytes = s.as_bytes();

        let n = s_bytes.len();
        
        let mut counter = 0;
        
        while window <= n {
            let mut idx = 0;
            
            while idx + window <= n {
                let sub_bytes = &s_bytes[idx..idx + window];

                let set: HashSet<u8> = HashSet::from_iter(sub_bytes.to_vec());
                
                if set.len() == 3 {
                    counter += 1;
                }
                idx += 1;
            }
            window += 1;
        }
        counter
    }
}
