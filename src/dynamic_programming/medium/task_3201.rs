use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn maximum_length_faster(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let patterns = vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 1]];
        for pattern in patterns {
            let mut cnt = 0;
            for num in &nums {
                if num % 2 == pattern[cnt % 2] {
                    cnt += 1;
                }
            }
            res = res.max(cnt);
        }
        res as i32
    }

    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        fn is_valid(temp: &Vec<i32>) -> bool {
            if temp.is_empty() {
                return true;
            }
            let mut vals = HashSet::new();
            for i in 0..temp.len() - 1 {
                vals.insert((temp[i] + temp[i + 1]) % 2);
            }
            vals.len() == 1
        }

        fn backtrack(
            subsequences: &mut Vec<Vec<i32>>,
            temp: &mut Vec<i32>,
            nums: &Vec<i32>,
            start: usize,
        ) {
            if is_valid(temp) {
                subsequences.push(temp.clone());
            }
            for i in start..nums.len() {
                temp.push(nums[i]);
                backtrack(subsequences, temp, nums, i + 1);
                temp.pop();
            }
        }

        let mut subsequences = vec![];

        backtrack(&mut subsequences, &mut vec![], &nums, 0);

        subsequences.iter().map(|x| x.len() as i32).max().unwrap()
    }
}
