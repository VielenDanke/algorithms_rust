pub struct Solution;

impl Solution {
    pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
        let len = (2 * n - 1) as usize;
        let mut result = vec![0; len];
        let mut used = vec![false; (n + 1) as usize];
        Solution::backtrack(&mut result, &mut used, n as usize, 0);
        result
    }

    fn backtrack(result: &mut Vec<i32>, used: &mut Vec<bool>, n: usize, index: usize) -> bool {
        let len = result.len();
        let mut index = index;

        while index < len && result[index] != 0 {
            index += 1;
        }

        if index == len {
            return true;
        }

        for i in (1..=n).rev() {
            if used[i] {
                continue;
            }

            if i == 1 {
                result[index] = 1;
                used[1] = true;
                if Solution::backtrack(result, used, n, index + 1) {
                    return true;
                }
                result[index] = 0;
                used[1] = false;
            } else {
                if index + i < len && result[index + i] == 0 {
                    result[index] = i as i32;
                    result[index + i] = i as i32;
                    used[i] = true;
                    if Solution::backtrack(result, used, n, index + 1) {
                        return true;
                    }
                    result[index] = 0;
                    result[index + i] = 0;
                    used[i] = false;
                }
            }
        }
        false
    }
}