use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub struct Solution;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    row: i32,
    col: i32,
    weight: i32,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        self.weight.cmp(&other.weight).reverse()
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Node {
    fn build(row: i32, col: i32, weight: i32) -> Node {
        Node {
            row,
            col,
            weight,
        }
    }
}

impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (heights.len(), heights[0].len());
        let mut weight_path = vec![vec![i32::MAX; m]; n];

        weight_path[0][0] = 0;

        let mut bh = BinaryHeap::new();

        bh.push(Node::build(0, 0, 0));

        let directions = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

        while let Some(Node { row, col, weight }) = bh.pop() {
            let (row_idx, col_idx) = (row as usize, col as usize);
            if row_idx == n - 1 && col_idx == m - 1 {
                break;
            }
            for dir in directions.iter() {
                let next_row = row + dir.0;
                let next_col = col + dir.1;
                let next_row_idx = next_row as usize;
                let next_col_idx = next_col as usize;

                if next_row < 0 || next_col < 0 || next_row_idx >= n || next_col_idx >= m {
                    continue;
                }
                let alt = weight.max((heights[row_idx][col_idx].abs_diff(heights[next_row_idx][next_col_idx])) as i32);
                if alt < weight_path[next_row_idx][next_col_idx] {
                    weight_path[next_row_idx][next_col_idx] = alt;
                    bh.push(Node::build(next_row, next_col, alt));
                }
            }
        }
        weight_path[n - 1][m - 1]
    }
}
