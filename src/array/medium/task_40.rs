pub struct Solution;

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let n = candidates.len();
        candidates.sort_unstable();
        Self::backtracking(&mut result, &mut candidates, &mut Vec::new(), target, n, 0);
        result
    }

    fn backtracking(result: &mut Vec<Vec<i32>>, candidates: &mut Vec<i32>, temp: &mut Vec<i32>, target: i32, n: usize, start: usize) {
        if target == 0 {
            result.push(temp.clone());
            return;
        }
        for i in start..n {
            let candidate = candidates[i];

            if i > start && candidate == candidates[i - 1] {
                continue;
            }
            if target - candidate < 0 {
                break;
            }
            temp.push(candidate);
            Self::backtracking(result, candidates, temp, target - candidate, n, i + 1);
            temp.remove(temp.len() - 1);
        }
    }
}