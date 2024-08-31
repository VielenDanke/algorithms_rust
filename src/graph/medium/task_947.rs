pub struct Solution;

impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let n = stones.len();
        let mut uf = crate::graph::UnionFind::build(n);

        for i in 0..n {
            let (x_i, y_i) = (stones[i][0], stones[i][1]);
            for j in (i + 1)..n {
                let (x_j, y_j) = (stones[j][0], stones[j][1]);
                if x_i == x_j || y_i == y_j {
                    uf.union(i, j);
                }
            }
        }
        n as i32 - uf.count
    }

    pub fn remove_stones_dfs(stones: Vec<Vec<i32>>) -> i32 {
        let n = stones.len();

        let mut graph = vec![vec![]; n];

        for i in 0..n {
            for j in i + 1.. n {
                if stones[i][0] == stones[j][0] || stones[i][1] == stones[j][1] {
                    graph[i].push(j);
                    graph[j].push(i);
                }
            }
        }
        let mut visited = vec![false; n];

        fn dfs(node: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>) -> i32 {
            visited[node] = true;
            let mut count = 1;
            for &neighbor in graph[node].iter() {
                if !visited[neighbor] {
                    count += dfs(neighbor, graph, visited);
                }
            }
            count
        }
        let mut max_removed = 0;

        for i in 0..n {
            if !visited[i] {
                let components = dfs(i, &graph, &mut visited);
                max_removed += components - 1;
            }
        }
        max_removed
    }
}