use crate::linked_list::ListNode;

pub struct Solution;

impl Solution {

    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        // critical point is a local maxima or local minima
        // find the minimum and the maximum distance between two critical points
        let mut points = Vec::new();

        let mut to_clone = head.clone();

        while let Some(mut node) = to_clone {
            points.push(node.val);
            to_clone = node.next.take();
        }

        let find_critical_points = |(idx, &v)| {
            if idx == 0 || idx == points.len() - 1 {
                None
            } else if v > points[idx - 1] && v > points[idx + 1] {
                Some(idx)
            } else if v < points[idx - 1] && v < points[idx + 1] {
                Some(idx)
            } else {
                None
            }
        };

        let filtered_points = points
            .iter()
            .enumerate()
            .filter_map(find_critical_points)
            .collect::<Vec<usize>>();

        if filtered_points.len() < 2 {
            return vec![-1, -1];
        }

        let mut minimum_distance = 1 << 30;
        let mut maximum_distance = filtered_points[filtered_points.len() - 1] - filtered_points[0];

        for i in 1..filtered_points.len() {
            minimum_distance = minimum_distance.min(filtered_points[i] - filtered_points[i - 1]);
        }

        vec![minimum_distance as i32, maximum_distance as i32]
    }
}
