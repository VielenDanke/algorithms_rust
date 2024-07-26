use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::hash::Hash;

pub struct Solution;

#[derive(PartialEq, Eq)]
pub struct WeightPair {
    node: i32,
    weight: i32,
}

impl WeightPair {
    fn build(node: i32, weight: i32) -> WeightPair {
        WeightPair { node, weight }
    }
}

impl Ord for WeightPair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self
            .weight
            .cmp(&other.weight)
    }
}

impl PartialOrd for WeightPair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let n = n as usize;
        let adjacency_list = {
            let mut res = vec![vec![]; n];
            for edge in edges {
                let u = edge[0] as usize;
                let v = edge[1] as usize;
                let w = edge[2];
                res[u].push((v, w));
                res[v].push((u, w));
            }
            res
        };
        let mut final_results = vec![];
        for end in 0..n {
            let mut distances = vec![i32::MAX; n];
            distances[end] = 0;
            let mut pq = BinaryHeap::new();
            pq.push((Reverse(0), end));
            while let Some((Reverse(distance), u)) = pq.pop() {
                for &(v, w) in &adjacency_list[u] {
                    let next_distance = distance + w;
                    if next_distance < distances[v] {
                        distances[v] = next_distance;
                        pq.push((Reverse(next_distance), v));
                    }
                }
            }
            let count = distances.into_iter().filter(|&a| a > 0 && a <= distance_threshold).count();
            final_results.push((count, Reverse(end)));
        }
        final_results.sort();
        (final_results.first().unwrap().1).0 as i32
    }
}
