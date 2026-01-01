pub struct Solution;

struct UnionFind {
    root: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            root: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while x != self.root[x] {
            self.root[x] = self.root[self.root[x]];
            x = self.root[x]
        }

        x
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x != root_y {
            if self.size[root_x] > self.size[root_y] {
                self.root[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
            } else {
                self.root[root_x] = root_y;
                self.size[root_y] += self.size[root_x];
            }
        }
    }
}

impl Solution {
    pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
        let r_len = row as usize;
        let c_len = col as usize;
        let total_cells = r_len * c_len;

        // 0 is top virtual node, total_cells + 1 is bottom virtual node
        let mut dsu = UnionFind::new(total_cells + 2);
        let mut grid = vec![vec![0; c_len]; r_len];
        let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        // Iterate backwards from the last day
        for i in (0..cells.len()).rev() {
            let r = (cells[i][0] - 1) as usize;
            let c = (cells[i][1] - 1) as usize;

            grid[r][c] = 1; // Mark as land

            let id1 = r * c_len + c + 1;

            // Check neighbors
            for &(dr, dc) in &dirs {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;

                if nr >= 0 && nr < row && nc >= 0 && nc < col {
                    let nr_u = nr as usize;
                    let nc_u = nc as usize;
                    if grid[nr_u][nc_u] == 1 {
                        dsu.union(id1, nr_u * c_len + nc_u + 1);
                    }
                }
            }

            // Connect to top virtual node
            if r == 0 {
                dsu.union(0, id1);
            }

            // Connect to bottom virtual node
            if r == r_len - 1 {
                dsu.union(total_cells + 1, id1);
            }

            // Check if top and bottom are connected
            if dsu.find(0) == dsu.find(total_cells + 1) {
                return i as i32;
            }
        }

        -1
    }
}