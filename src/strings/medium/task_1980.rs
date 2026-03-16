use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let mut result = Vec::new();
        let mut idx = 0;

        for num in nums {
            if num.as_bytes()[idx] == b'1' {
                result.push(b'0');
            } else {
                result.push(b'1');
            }
            idx += 1;
        }
        if let Ok(result) = String::from_utf8(result) {
            result
        } else {
            String::new()
        }
    }

    pub fn find_different_binary_string_brute_force(nums: Vec<String>) -> String {
        fn find_first_candidate(set: &mut HashSet<Vec<u8>>, temp: &mut Vec<u8>, n: usize) -> Vec<u8> {
            if temp.len() == n {
                return if set.insert(temp.clone()) {
                    temp.clone()
                } else {
                    Vec::new()
                }
            }
            temp.push(b'1');
            let result = find_first_candidate(set, temp, n);
            if !result.is_empty() {
                return result;
            }
            temp.pop();
            temp.push(b'0');
            let result = find_first_candidate(set, temp, n);
            if !result.is_empty() {
                return result;
            }
            temp.pop();
            Vec::new()
        }
        let mut set = HashSet::new();
        for num in nums.iter() {
            set.insert(num.as_bytes().to_vec());
        }
        String::from_utf8(find_first_candidate(&mut set, &mut Vec::new(), nums[0].len())).unwrap()
    }
}