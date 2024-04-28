use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub struct Solution;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    cost: i32,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn sum_of_distances_in_tree(n: i32, mut edges: Vec<Vec<i32>>) -> Vec<i32> {
        let (mut count, mut result, edges) = (
            vec![0; n as usize],
            vec![0; n as usize],
            Solution::collect_edges(n, edges)
        );

        fn prev_traverse(cur: i32, parent: i32, graph: &Vec<Vec<i32>>, count: &mut Vec<i32>, result: &mut Vec<i32>) {
            let cur = cur as usize;
            count[cur] = 1;
            if let Some(children) = graph.get(cur) {
                for &child in children {
                    if child != parent {
                        prev_traverse(child, cur as i32, graph, count, result);
                        let child = child as usize;
                        count[cur] += count[child];
                        result[cur] += result[child] + count[child];
                    }
                }
            }
        }

        fn post_traverse(cur: i32, parent: i32, n: i32, count: &Vec<i32>, result: &mut Vec<i32>, graph: &Vec<Vec<i32>>) {
            let cur = cur as usize;
            if let Some(children) = graph.get(cur) {
                for &child in children {
                    if child != parent {
                        let child = child as usize;
                        result[child] = result[cur] - count[child] + (n - count[child]);
                        post_traverse(child as i32, cur as i32, n, count, result, graph);
                    }
                }
            }
        }

        prev_traverse(0, -1, &edges, &mut count, &mut result);
        post_traverse(0, -1, n, &count, &mut result, &edges);

        result
    }

    // ---------------------------------------------------------------------------------------------------

    pub fn sum_of_distances_in_tree_tle_dijkstra(n: i32, mut edges: Vec<Vec<i32>>) -> Vec<i32> {
        edges = Solution::collect_edges(n, edges);
        let mut result = vec![0; n as usize];
        for start in 0..n as usize {
            for end in 0..n as usize {
                if start != end {
                    result[start] += Solution::traverse(start, end, &edges).unwrap_or(0);
                }
            }
        }
        result
    }

    fn traverse(start: usize, end: usize, edges: &Vec<Vec<i32>>) -> Option<i32> {
        let mut dist: Vec<_> = (0..edges.len()).map(|_| i32::MAX).collect();

        let mut heap = BinaryHeap::new();

        // We're at `start`, with a zero cost
        dist[start] = 0;
        heap.push(Node { cost: 0, position: start });

        // Examine the frontier with lower cost nodes first (min-heap)
        while let Some(Node { cost, position }) = heap.pop() {
            // Alternatively we could have continued to find all shortest paths
            if position == end { return Some(cost); }

            // Important as we may have already found a better way
            if cost > dist[position] { continue; }

            // For each node we can reach, see if we can find a way with
            // a lower cost going through this node
            for edge in &edges[position] {
                let next = Node { cost: cost + 1, position: *edge as usize };

                // If so, add it to the frontier and continue
                if next.cost < dist[next.position] {
                    heap.push(next);
                    // Relaxation, we have now found a better way
                    dist[next.position] = next.cost;
                }
            }
        }
        // Goal not reachable
        None
    }

    fn collect_edges(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut transformed_edges = vec![vec![]; n as usize];

        for edge in edges {
            transformed_edges[edge[0] as usize].push(edge[1]);
            transformed_edges[edge[1] as usize].push(edge[0]);
        }
        transformed_edges
    }
}
