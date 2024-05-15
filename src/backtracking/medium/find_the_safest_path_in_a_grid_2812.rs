use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};

pub struct Solution;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    x: i32,
    y: i32,
    safeness: i32,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        self.safeness.cmp(&other.safeness)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Node {
    pub fn build(x: i32, y: i32, safeness: i32) -> Node {
        Node {
            x, y, safeness
        }
    }
}

impl Solution {
    pub fn maximum_safeness_factor(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let in_bounds = |r: usize, c: usize| { r < n && c < n };
        let mut queue = VecDeque::new();
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    queue.push_back((i, j, 1));
                    grid[i][j] = 0;
                } else {
                    grid[i][j] = -1;
                }
            }
        }
        while let Some((i, j, safety)) = queue.pop_front() {
            let expand = [(i+1, j), (i-1, j), (i, j+1), (i, j-1)];
            for (r, c) in expand {
                if in_bounds(r, c) && grid[r][c] == -1 {
                    grid[r][c] = safety;
                    queue.push_back((r, c, safety + 1));
                }
            }
        }
        let mut min_safety = grid[0][0];
        queue.push_back((0, 0, grid[0][0]));
        while let Some((i, j, safety)) = queue.pop_front() {
            min_safety = i32::min(min_safety, safety);
            if (i, j) == (n-1, n-1) { break };
            let expand = [(i+1, j), (i-1, j), (i, j+1), (i, j-1)];
            for (r, c) in expand {
                if in_bounds(r, c) && grid[r][c] != -1 {
                    let safety = grid[r][c];
                    grid[r][c] = -1;
                    if safety < min_safety { queue.push_back((r, c, safety)); }
                    else { queue.push_front((r, c, safety)); }
                }
            }
        }
        min_safety
    }

    pub fn maximum_safeness_factor_dijkstra(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut thieves = Vec::new();
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 1 {
                    thieves.push((i, j));
                    grid[i][j] = 0;
                }
            }
        }
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] != 1 {
                    let mut result = i32::MAX;
                    for (ii, jj) in thieves.iter() {
                        result = result.min(((i as i32).abs_diff((*ii) as i32) + (j as i32).abs_diff((*jj as i32))) as i32);
                    }
                    grid[i][j] = result;
                }
            }
        }
        let (n, m) = (grid.len() as i32, grid[0].len() as i32);

        let mut bh = BinaryHeap::new();

        bh.push(Node::build(0, 0, grid[0][0]));

        let mut safeness_visited = vec![vec![i32::MAX; grid[0].len()]; grid.len()];

        let next_points = vec![(-1, 0), (1, 0), (0, 1), (0, -1)];

        while let Some(next) = bh.pop() {
            if next.x == n - 1 && next.y == m - 1 {
                return next.safeness;
            }
            if next.safeness > safeness_visited[next.x as usize][next.y as usize] {
                continue;
            }
            for (i, j) in next_points.iter() {
                let next_row = *i + next.x;
                let next_col = *j + next.y;

                if next_row < 0 || next_col < 0 || next_row >= n || next_col >= m {
                    continue;
                }
                let next_safeness = grid[next_row as usize][next_col as usize].min(next.safeness);
                if safeness_visited[next_row as usize][next_col as usize] > next_safeness {
                    bh.push(Node::build(next_row, next_col, next_safeness));
                    safeness_visited[next_row as usize][next_col as usize] = next_safeness;
                }
            }
        }
        safeness_visited[(n - 1) as usize][(m - 1) as usize]
    }

    pub fn maximum_safeness_factor_brute_force(grid: Vec<Vec<i32>>) -> i32 {
        // starting point at (0, 0)
        // find all thieves
        // calculate safeness factor to the thieves after calculating a path
        let mut thieves = Vec::new();
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 1 {
                    thieves.push((i as i32, j as i32));
                }
            }
        }
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        visited[0][0] = true;
        let mut path = Vec::new();
        path.push((0, 0));
        return Solution::backtrack(&grid, &mut visited, &mut path, &thieves, 0, 0);
    }

    fn backtrack(grid: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>, path: &mut Vec<(i32, i32)>, thieves: &Vec<(i32, i32)>, row: i32, col: i32) -> i32 {
        if row as usize == grid.len() - 1 && col as usize == grid[row as usize].len() - 1 {
            let mut result = i32::MAX;
            for v in path.iter() {
                for t in thieves.iter() {
                    result = result.min((v.0.abs_diff(t.0) + v.1.abs_diff(t.1)) as i32);
                }
            }
            return result;
        }
        let mut result = i32::MIN;
        for (i, j) in vec![(-1, 0), (1, 0), (0, 1), (0, -1)].iter() {
            let next_row = row + *i;
            let next_col = col + *j;
            if next_row < 0 || next_col < 0 || next_row as usize >= grid.len() || next_col as usize >= grid[next_row as usize].len() {
                continue;
            }
            if visited[next_row as usize][next_col as usize] {
                continue;
            }
            visited[next_row as usize][next_col as usize] = true;
            path.push((next_row, next_col));
            result = result.max(Solution::backtrack(grid, visited, path, thieves, next_row, next_col));
            visited[next_row as usize][next_col as usize] = false;
            path.remove(path.len() - 1);
        }
        result
    }
}
