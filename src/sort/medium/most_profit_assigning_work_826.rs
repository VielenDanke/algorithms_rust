pub struct Solution;

impl Solution {

    pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
        let mut result = 0;
        for &worker_capacity in worker.iter() {
            let mut max_seen_profit = 0;
            for (idx, &current_difficulty) in difficulty.iter().enumerate() {
                if current_difficulty <= worker_capacity {
                    max_seen_profit = max_seen_profit.max(profit[idx]);
                }
            }
            result += max_seen_profit;
        }
        result
    }

    pub fn max_profit_assignment_sort(difficulty: Vec<i32>, profit: Vec<i32>, mut worker: Vec<i32>) -> i32 {
        let (n, m) = (profit.len(), worker.len());

        worker.sort_unstable();

        let mut positions = (0..n).collect::<Vec<usize>>();

        positions.sort_unstable_by(|&a, &b| difficulty[a].cmp(&difficulty[b]));

        let (mut diff_profit_idx, mut worker_idx) = (0, 0);
        let mut result = 0;
        let mut max = 0;

        while worker_idx < m {
            while diff_profit_idx < n && difficulty[positions[diff_profit_idx]] <= worker[worker_idx] {
                max = max.max(profit[positions[diff_profit_idx]]);
                diff_profit_idx += 1;
            }

            result += max;
            worker_idx += 1;
        }

        result
    }
}
