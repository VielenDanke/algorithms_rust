/*
There is a dungeon with n x m rooms arranged as a grid.

You are given a 2D array moveTime of size n x m,
where moveTime[i][j] represents the minimum time in seconds when you can start moving to that room.
You start from the room (0, 0) at time t = 0 and can move to an adjacent room.
Moving between adjacent rooms takes exactly one second.

Return the minimum time to reach the room (n - 1, m - 1).

Two rooms are adjacent if they share a common wall, either horizontally or vertically.
 */
use std::cmp::{max, Reverse};
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn min_time_to_reach_binary_heap(move_time: Vec<Vec<i32>>) -> i32 {
        let n = move_time.len();
        let m = move_time[0].len();
        let inf = i32::MAX / 2;
        let mut d = vec![vec![inf; m]; n];
        let mut visited = vec![vec![false; m]; n];
        let directories = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        d[0][0] = 0;
        
        let mut q = BinaryHeap::new();
        
        q.push(Reverse((0, 0usize, 0usize))); // (dis, x, y)

        while let Some(Reverse((_, x, y))) = q.pop() {
            if visited[x][y] {
                continue;
            }
            visited[x][y] = true;
            
            for &(dx, dy) in directories.iter() {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx < 0 || nx >= n as isize || ny < 0 || ny >= m as isize {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                
                let dist = max(d[x][y], move_time[nx][ny]) + 1;
                
                if d[nx][ny] > dist {
                    d[nx][ny] = dist;
                    q.push(Reverse((dist, nx, ny)));
                }
            }
        }

        d[n - 1][m - 1]
    }

    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let possible_steps = vec![vec![1, 0], vec![0, 1], vec![-1, 0], vec![0, -1]];

        fn backtrack(
            row: usize,
            col: usize,
            time: i32,
            prev_time: i32,
            move_time: &Vec<Vec<i32>>,
            possible_steps: &Vec<Vec<i32>>,
            visited: &mut Vec<Vec<bool>>,
        ) -> i32 {
            if row == move_time.len() - 1 && col == move_time[0].len() - 1 {
                return time;
            }
            let mut min_time = i32::MAX;

            for step in possible_steps.iter() {
                if row as i32 + step[0] >= 0
                    && row as i32 + step[0] < move_time.len() as i32
                    && col as i32 + step[1] >= 0
                    && col as i32 + step[1] < move_time[0].len() as i32
                    && !visited[(row as i32 + step[0]) as usize][(col as i32 + step[1]) as usize]
                {
                    visited[(row as i32 + step[0]) as usize][(col as i32 + step[1]) as usize] =
                        true;

                    min_time = min_time.min(backtrack(
                        (row as i32 + step[0]) as usize,
                        (col as i32 + step[1]) as usize,
                        time + (move_time[row][col] - prev_time) + 1,
                        move_time[row][col],
                        move_time,
                        possible_steps,
                        visited,
                    ));
                }
            }
            min_time
        }

        backtrack(
            0,
            0,
            move_time[0][0],
            move_time[0][0],
            &move_time,
            &possible_steps,
            &mut vec![vec![false; move_time[0].len()]; move_time.len()],
        )
    }
}
