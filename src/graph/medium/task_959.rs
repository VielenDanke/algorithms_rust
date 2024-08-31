pub struct Solution;

impl Solution {
    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        let rows = grid.len();
        let dots = rows + 1;

        let mut uf = crate::graph::UnionFind::build(dots);

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