use std::collections::{HashMap, VecDeque, HashSet};

pub struct Solution {}

impl Solution {

    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let should_remain = 2;

        if n < should_remain {
            return (0..n).collect::<Vec<i32>>();
        }
        let mut graph = Solution::create_graph(edges);
        let mut leaves = graph
            .iter()
            .filter_map(|(i, set)| if set.len() == 1 { Some(*i) } else { None })
            .collect::<Vec<i32>>();

        let mut remaining_nodes = n;

        while remaining_nodes > should_remain {
            remaining_nodes -= leaves.len() as i32;

            let mut new_leaves = vec![];

            for leaf in leaves {
                let neighbour = *graph[&leaf].iter().next().unwrap();
                graph.get_mut(&neighbour).unwrap().remove(&leaf);

                if graph[&neighbour].len() == 1 {
                    new_leaves.push(neighbour);
                }
            }

            leaves = new_leaves;
        }
        leaves
    }

    pub fn find_min_height_trees_bfs(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        fn bfs(node: i32, graph: &HashMap<i32, HashSet<i32>>) -> i32 {
            let mut queue = VecDeque::new();

            queue.push_back(node);

            let mut visited = vec![false; graph.len()];

            visited[node as usize] = true;

            let mut max_depth = 0;

            while !queue.is_empty() {
                for _ in 0..queue.len() {
                    let elem = queue.pop_front().unwrap();

                    for node in graph.get(&elem).unwrap_or(&Vec::new()).iter() {
                        if !visited[*node as usize] {
                            visited[*node as usize] = true;
                            queue.push_back(*node);
                        }
                    }
                }
                max_depth += 1;
            }
            max_depth
        }
        if edges.is_empty() {
            return Vec::new();
        }
        let graph = Solution::create_graph(edges);
        let mut min = 1 << 30;
        let mut result = Vec::new();
        for root in 0..n {
            let current_max = bfs(root, &graph);
            if current_max < min {
                min = current_max;
                result.clear();
                result.push(root);
            } else if current_max == min {
                result.push(root);
            }
        }
        result
    }

    pub fn find_min_height_trees_dfs_tle(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        fn dfs(node: i32, graph: &HashMap<i32, HashSet<i32>>, visited: &mut Vec<i32>) -> i32 {
            if visited[node as usize] != -1 {
                return 0;
            }
            if !graph.contains_key(&node) {
                return 1;
            }
            visited[node as usize] = 1;
            let mut current_max = -1 << 30;
            for current_node in graph[&node].iter() {
                current_max = current_max.max(dfs(*current_node, graph, visited));
            }
            let increased_max = current_max + 1;
            increased_max
        }
        let graph = Solution::create_graph(edges);
        let mut min = 1 << 30;
        let mut result = Vec::new();
        for root in 0..n {
            let mut visited = vec![-1; n as usize];
            let current_max = dfs(root, &graph, &mut visited);
            if current_max < min {
                min = current_max;
                result.clear();
                result.push(root);
            } else if current_max == min {
                result.push(root);
            }
        }
        result
    }

    fn create_graph(edges: Vec<Vec<i32>>) -> HashMap<i32, HashSet<i32>> {
        let mut graph = HashMap::new();

        for edge in edges {
            graph.entry(edge[0]).or_insert(HashSet::new()).insert(edge[1]);
            graph.entry(edge[1]).or_insert(HashSet::new()).insert(edge[0]);
        }
        graph
    }
}
