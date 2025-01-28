use std::collections::HashMap;

pub struct Solution;

impl Solution {
    fn is_prerequisite(
        adj_list: &HashMap<usize, Vec<usize>>,
        visited: &mut Vec<bool>,
        src: usize,
        target: usize,
    ) -> bool {
        // Mark the current node as visited
        visited[src] = true;

        // If src is the same as target, return true
        if src == target {
            return true;
        }

        // Check all neighbors of the current node
        if let Some(neighbors) = adj_list.get(&src) {
            for &neighbor in neighbors {
                if !visited[neighbor] {
                    if Self::is_prerequisite(adj_list, visited, neighbor, target) {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn check_if_prerequisite(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        // Convert num_courses to usize for indexing
        let num_courses = num_courses as usize;

        // Build the adjacency list
        let mut adj_list: HashMap<usize, Vec<usize>> = HashMap::new();
        
        for edge in prerequisites {
            let src = edge[0] as usize;
            let dest = edge[1] as usize;
            adj_list.entry(src).or_default().push(dest);
        }

        // Process each query
        let mut result = Vec::new();

        for query in queries {
            let src = query[0] as usize;
            let target = query[1] as usize;

            // Reset visited array for each query
            let mut visited = vec![false; num_courses];
            result.push(Self::is_prerequisite(&adj_list, &mut visited, src, target));
        }

        result
    }
}
