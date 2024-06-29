use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut graph = vec![Vec::new(); n as usize];

        for edge in edges.iter() {
            graph[edge[0] as usize].push(edge[1]);
        }

        let mut result = vec![vec![]; n as usize];

        for i in 0..n {
            Self::dfs(i, i, &graph, &mut result);
        }

        result
    }

    fn dfs(next: i32, parent: i32, graph: &Vec<Vec<i32>>, result: &mut Vec<Vec<i32>>) {
        for &next_node in graph[next as usize].iter() {
            if let Some(&last_node) = result[next_node as usize].last() {
                if parent == last_node {
                    continue;
                }
            }
            result[next_node as usize].push(parent);
            Self::dfs(next_node, parent, graph, result);
        }
    }

    pub fn get_ancestors_stack(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;

        let mut graph = vec![vec![]; n];

        for edge in edges {
            graph[edge[0] as usize].push(edge[1] as usize);
        }

        let mut ancestors = vec![vec![]; n];

        for ancestor in 0..n {
            let mut stack = graph[ancestor].clone();

            while let Some(node) = stack.pop() {
                if ancestors[node].last().is_some_and(|&node| node == ancestor as i32) {
                    continue;
                }

                ancestors[node].push(ancestor as i32);
                stack.extend_from_slice(&graph[node]);
            }
        }

        ancestors
    }

    pub fn get_ancestors_tle(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut collector = HashMap::new();

        (0..n).into_iter().for_each(|v| {
            collector.entry(v).or_insert(HashSet::new());
        });

        edges.iter().for_each(|edge| {
            collector.entry(edge[0]).or_insert(HashSet::new()).insert(edge[1]);
        });

        let mut result = vec![Vec::new(); n as usize];
        let mut candidates = Vec::new();

        for (&k, v) in collector.iter() {
            if v.is_empty() {
                candidates.push(k);
            }
        }

        while !collector.is_empty() {
            let m = candidates.len();

            for &candidate in candidates.iter() {
                for (&k, mut v) in collector.iter_mut() {
                    if k != candidate && v.contains(&candidate) {
                        result[candidate as usize].push(k);
                        v.remove(&candidate);
                    }
                }
                collector.remove(&candidate);
            }

            collector.iter().for_each(|(&k, v)| {
                if v.is_empty() {
                    candidates.push(k);
                }
            });

            candidates = candidates[m..candidates.len()].to_vec();
        }

        let mut new_result = vec![HashSet::new(); n as usize];

        for (i, v) in result.iter().enumerate() {
            let mut temp = HashSet::new();
            for &next in v.iter() {
                Self::dfs_topological(next, &result).iter().for_each(|&v| {
                    temp.insert(v);
                });
                v.iter().for_each(|&v| {
                    temp.insert(v);
                });
            }
            new_result[i] = temp;
        }

        new_result.into_iter().map(|v| {
            let mut arr = Vec::from_iter(v);
            arr.sort_unstable();
            arr
        }).collect::<Vec<Vec<i32>>>()
    }

    fn dfs_topological(next: i32, result: &Vec<Vec<i32>>) -> Vec<i32> {
        if result[next as usize].is_empty() {
            return Vec::new();
        }
        let mut total = Vec::new();
        total.push(next);
        for &v in result[next as usize].iter() {
            let temp = Self::dfs_topological(v, &result);
            temp.iter().for_each(|&v| total.push(v));
            total.push(v);
        }
        total
    }
}
