pub struct Solution;

impl Solution {
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        fn get_distances(edges: &Vec<i32>, start: usize) -> Vec<i32> {
            let mut edges_distances = vec![-1; edges.len()];
            let mut distance = 0;
            let mut current_node = start;
            while edges_distances[current_node] == -1 {
                edges_distances[current_node] = distance;
                distance += 1;
                let next = edges[current_node];
                if next == -1 {
                    break;
                }
                current_node = next as usize;
            }
            edges_distances
        }

        let dist1 = get_distances(&edges, node1 as usize);
        let dist2 = get_distances(&edges, node2 as usize);
        
        let mut result = -1;
        let mut min_distance = i32::MAX;

        for i in 0..edges.len() {
            if dist1[i] != -1 && dist2[i] != -1 {
                let max_dist = dist1[i].max(dist2[i]);
                if max_dist < min_distance {
                    min_distance = max_dist;
                    result = i as i32;
                }
            }
        }
        result
    }
}