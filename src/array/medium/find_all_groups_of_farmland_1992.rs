struct Solution {}

/// Array, Depth-First Search, Breadth-First Search, Matrix
impl Solution {
    pub fn find_farmland_no_recursion(mut land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut results: Vec<Vec<i32>> = Vec::new();

        let (n, m) = (land.len(), land[0].len());

        for i in 0..n {
            for j in 0..m {
                if land[i][j] == 1 {
                    let (mut mut_i, mut mut_j) = (i, j);

                    while mut_i < n && land[mut_i][j] == 1 {
                        mut_j = j;
                        while mut_j < m && land[mut_i][mut_j] == 1 {
                            land[mut_i][mut_j] = 0;
                            mut_j += 1;
                        }
                        mut_i += 1;
                    }
                    results.push(vec![i as i32, j as i32, mut_i as i32 - 1, mut_j as i32 - 1]);
                }
            }
        }
        results
    }

    pub fn find_farmland(mut land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn dfs(land: &mut Vec<Vec<i32>>, directions: &Vec<(i32, i32)>, row: i32, col: i32, n: i32, m: i32) -> (i32, i32) {
            if row < 0 || col < 0 || row >= n || col >= m {
                return (-1, -1);
            }
            let current = land[row as usize][col as usize];
            land[row as usize][col as usize] = 0;
            if current == 0 {
                return (-1, -1);
            }
            let mut max_r = row;
            let mut max_c = col;
            for dir in directions {
                let resp = dfs(land, directions, row + dir.0, col + dir.1, n, m);
                max_r = max_r.max(resp.0);
                max_c = max_c.max(resp.1);
            }
            (max_r, max_c)
        }
        let (n, m) = (land.len(), land[0].len());
        let directions = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

        let mut result = Vec::new();

        for i in 0..n {
            for j in 0..m {
                let mut current = Vec::new();
                if land[i][j] == 1 {
                    current.push(i as i32);
                    current.push(j as i32);

                    let max_coordinates = dfs(&mut land, &directions, i as i32, j as i32, n as i32, m as i32);

                    current.push(max_coordinates.0);
                    current.push(max_coordinates.1);

                    result.push(current);
                }
            }
        }
        result
    }
}
