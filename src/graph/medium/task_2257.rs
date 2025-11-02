pub struct Solution;


impl Solution {

    const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    fn dfs(store: &mut Vec<Vec<i32>>, dir: &(i32, i32), x: i32, y: i32) {
        if x < 0 || y < 0 || x as usize >= store.len() || y as usize >= store[0].len() {
            return;
        } else if store[x as usize][y as usize] >= 2 {
            return;
        } else {
            store[x as usize][y as usize] = 1;
            Self::dfs(store, dir, x + dir.0, y + dir.1)
        }
    }

    // guard = 2
    // wall = 3
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let mut store = vec![vec![0; n as usize]; m as usize];

        for guard in guards.iter() {
            store[guard[0] as usize][guard[1] as usize] = 2;
        }
        for wall in walls.iter() {
            store[wall[0] as usize][wall[1] as usize] = 3;
        }

        for i in 0..m {
            for j in 0..n {
                if store[i as usize][j as usize] == 2 {
                    for dir in Self::DIRECTIONS.iter() {
                        Self::dfs(&mut store, dir, i + dir.0, j + dir.1);
                    }
                }
            }
        }

        store.iter().flatten().filter(|&&x| x == 0).count() as i32
    }
}
