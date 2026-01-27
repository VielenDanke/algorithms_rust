use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i64,
    node: usize,
    used: usize,
}

// Manually implement Ord so the BinaryHeap becomes a min-heap
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Solution;

impl Solution {
    pub fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut adj_out = vec![vec![]; n];
        let mut adj_in = vec![vec![]; n];

        for e in edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            let w = e[2] as i64;
            adj_out[u].push((v, w));
            adj_in[v].push((u, w));
        }

        const INF: i64 = 1e18 as i64;
        let mut dist = vec![[INF; 2]; n];
        let mut pq = BinaryHeap::new();

        dist[0][0] = 0;
        pq.push(State { cost: 0, node: 0, used: 0 });

        while let Some(State { cost, node, used }) = pq.pop() {
            if cost > dist[node][used] {
                continue;
            }

            // Standard forward edges
            for &(v, w) in &adj_out[node] {
                if dist[v][used] > cost + w {
                    dist[v][used] = cost + w;
                    pq.push(State { cost: dist[v][used], node: v, used });
                }
            }

            // Using a reversed edge (only if not already used)
            if used == 0 {
                for &(v, w) in &adj_in[node] {
                    // Logic based on original Java: cost + 2*w and state transition to 'used'
                    if dist[v][0] > cost + 2 * w {
                        dist[v][0] = cost + 2 * w;
                        pq.push(State { cost: dist[v][0], node: v, used: 0 });
                    }
                }
            }
        }

        let ans = dist[n - 1][0].min(dist[n - 1][1]);
        if ans >= INF { -1 } else { ans as i32 }
    }
}