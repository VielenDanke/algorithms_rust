pub struct Solution;

struct UnionFind {
    id: Vec<usize>,
    rank: Vec<usize>,
    count: i32,
}

impl UnionFind {
    fn build(n: usize) -> UnionFind {
        let mut rank = vec![0; n * n];
        let mut parent = vec![0; n * n];
        let mut count = 0;

        for i in 0..parent.len() {
            parent[i] = i;
            rank[i] = 1;
        }

        UnionFind {
            id: parent,
            rank,
            count,
        }
    }

    fn union(&mut self, x: usize, y: usize) {
        let parent_x = self.find(x);
        let parent_y = self.find(y);
        if parent_x == parent_y {
            self.count += 1;
        } else {
            if self.rank[parent_x] > self.rank[parent_y] {
                self.id[parent_y] = parent_x;
            } else if self.rank[parent_x] < self.rank[parent_y] {
                self.id[parent_x] = parent_y;
            } else {
                self.id[parent_y] = parent_x;
                self.rank[parent_x] += 1;
            }
        }
    }

    fn find(&mut self, mut p: usize) -> usize {
        while p != self.id[p] {
            self.id[p] = self.id[self.id[p]];
            p = self.id[p];
        }
        p
    }
}

impl Solution {
    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        let rows = grid.len();
        let dots = rows + 1;

        let mut uf = UnionFind::build(dots);

        for i in 0..dots {
            for j in 0..dots {
                if i == 0 || j == 0 || i == rows || j == rows {
                    let cells = i * dots + j;
                    uf.union(0, cells);
                }
            }
        }

        for (i, gr) in grid.iter().enumerate() {
            for (j, ch) in gr.chars().enumerate() {
                if ch == '\\' {
                    let (cell_1, cell_2) = (i * dots + j, (i + 1) * dots + (j + 1));
                    uf.union(cell_1, cell_2);
                } else if ch == '/' {
                    let (cell_1, cell_2) = ((i + 1) * dots + j, i * dots + (j + 1));
                    uf.union(cell_1, cell_2);
                }
            }
        }

        uf.count
    }
}