use std::cmp;

pub struct Solution;

impl Solution {
    pub fn max_profit(
        n: i32,
        present: Vec<i32>,
        future: Vec<i32>,
        hierarchy: Vec<Vec<i32>>,
        budget: i32,
    ) -> i32 {
        let n = n as usize;
        let b_limit = budget as usize;

        // Build adjacency list (Tree)
        // Adjusting 1-based input to 0-based index
        let mut tree: Vec<Vec<usize>> = vec![vec![]; n];
        for edge in hierarchy {
            let u = (edge[0] - 1) as usize;
            let v = (edge[1] - 1) as usize;
            tree[u].push(v);
        }

        // dp[node][parent_bought][budget]
        let mut dp = vec![vec![vec![0; b_limit + 1]; 2]; n];

        // Start DFS from root (node 0)
        Self::dfs(0, &tree, &present, &future, b_limit, &mut dp);

        // The answer is the max possible profit at the root
        // assuming parent (non-existent) didn't buy.
        let mut ans = 0;
        for b in 0..=b_limit {
            ans = cmp::max(ans, dp[0][0][b]);
        }
        ans
    }

    // Merge two knapsack arrays: O(B^2)
    fn merge(a: &[i32], b: &[i32], limit: usize) -> Vec<i32> {
        let mut c = vec![i32::MIN / 2; limit + 1];

        for i in 0..=limit {
            if a[i] < 0 {
                continue;
            }
            for j in 0..=(limit - i) {
                c[i + j] = cmp::max(c[i + j], a[i] + b[j]);
            }
        }
        c
    }

    fn dfs(
        u: usize,
        tree: &Vec<Vec<usize>>,
        present: &Vec<i32>,
        future: &Vec<i32>,
        limit: usize,
        dp: &mut Vec<Vec<Vec<i32>>>,
    ) {
        // Recursively process children first (Post-order traversal)
        for &v in &tree[u] {
            Self::dfs(v, tree, present, future, limit, dp);
        }

        // Iterate over the state of the parent (0 = not bought, 1 = bought)
        for parent_bought in 0..=1 {
            let price = if parent_bought == 1 {
                present[u] / 2
            } else {
                present[u]
            };

            let profit = future[u] - price;

            // --- Option 1: Skip buying current node u ---
            // Initialize with 0s (representing 0 profit for existing budget)
            let mut skip = vec![0; limit + 1];

            for &v in &tree[u] {
                // If we skip u, u is NOT a "bought parent" for v, so we use dp[v][0]
                // We create a temporary slice to avoid borrow checker conflicts
                let child_dp = &dp[v][0].clone();
                skip = Self::merge(&skip, child_dp, limit);
            }

            // --- Option 2: Buy current node u ---
            let mut take = vec![i32::MIN / 2; limit + 1];

            if (price as usize) <= limit {
                // Base represents accumulating results from children given u is bought
                let mut base = vec![0; limit + 1];
                for &v in &tree[u] {
                    // If we buy u, u IS a "bought parent" for v, so we use dp[v][1]
                    let child_dp = &dp[v][1].clone();
                    base = Self::merge(&base, child_dp, limit);
                }

                // Shift the base results by the price of u and add u's profit
                for b in (price as usize)..=limit {
                    if base[b - price as usize] > i32::MIN / 2 {
                        take[b] = base[b - price as usize] + profit;
                    }
                }
            }

            // --- Store Best of Take / Skip ---
            for b in 0..=limit {
                dp[u][parent_bought][b] = cmp::max(skip[b], take[b]);
            }
        }
    }
}