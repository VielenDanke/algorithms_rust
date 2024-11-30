use std::collections::HashMap;

pub struct Solution;

impl Solution {
    fn calculate_shortest_distance_tle(graph: &HashMap<i32, Vec<i32>>, current: i32, n: i32, length: i32) -> i32 {
        if current == n - 1 {
            return length;
        }
        let neighbors = graph.get(&current).unwrap();

        let mut max_length = 1 << 30;

        for &neighbor in neighbors.iter() {
            max_length = max_length.min(Self::calculate_shortest_distance(graph, neighbor, n, length + 1));
        }
        max_length
    }

    fn calculate_shortest_distance(graph: &HashMap<i32, Vec<i32>>, current: i32, n: i32, length: i32) -> i32 {
        if current == n - 1 {
            return length;
        }
        let neighbors = graph.get(&current).unwrap();

        let mut find_max_next = 0;

        for &neighbor in neighbors.iter() {
            find_max_next = find_max_next.max(neighbor);
        }
        Self::calculate_shortest_distance(graph, find_max_next, n, length + 1)
    }

    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        // after each query we need to update the path

        let mut graph = HashMap::new();

        for i in 0..n {
            graph.entry(i)
                .or_insert(Vec::new())
                .push(i + 1);
        }
        let mut result = vec![0; queries.len()];

        for (i, query) in queries.iter().enumerate() {
            graph.entry(query[0])
                .or_default()
                .push(query[1]);

            result[i] = Self::calculate_shortest_distance(&graph, 0, n, 0);
        }
        result
    }
}