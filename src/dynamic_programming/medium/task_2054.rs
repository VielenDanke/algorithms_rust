pub struct Solution;

impl Solution {
    pub fn max_two_events(mut events: Vec<Vec<i32>>) -> i32 {
        events.sort_unstable_by_key(|e| e[0]);

        let n = events.len();

        let mut memo = vec![vec![-1; 3]; n];

        Self::dfs(&events, 0, 0, &mut memo)
    }

    fn dfs(events: &Vec<Vec<i32>>, idx: usize, count: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
        if count == 2 || idx >= events.len() {
            return 0;
        }

        if memo[idx][count] != -1 {
            return memo[idx][count];
        }

        let skip = Self::dfs(events, idx + 1, count, memo);

        let target_start = events[idx][1] + 1;

        let next_idx = events.partition_point(|e| e[0] < target_start);

        let pick = events[idx][2] + Self::dfs(events, next_idx, count + 1, memo);

        let res = skip.max(pick);
        memo[idx][count] = res;

        res
    }
}