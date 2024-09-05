use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let modes = vec![(0,1), (1,0), (0,-1), (-1,0)];
        let mut direction = 0;  // start facing North
        let (mut x, mut y) = (0, 0);

        // Convert obstacles to a set of string coordinates for quick lookup
        let obstacle_set: HashSet<(i32, i32)> = obstacles.iter().map(|v| (v[0], v[1])).collect();

        let mut max_dist_sq = 0;

        for &command in &commands {
            if command == -2 {
                // Turn left
                direction = (direction + 3) % 4;
            } else if command == -1 {
                // Turn right
                direction = (direction + 1) % 4;
            } else {
                // Move forward
                for _ in 0..command {
                    let nx = x + modes[direction].0;
                    let ny = y + modes[direction].1;
                    // Check for obstacle
                    if obstacle_set.contains(&(nx, ny)) {
                        break;  // stop moving if there's an obstacle
                    }
                    x = nx;
                    y = ny;
                    max_dist_sq = max_dist_sq.max(x * x + y * y);
                }
            }
        }
        max_dist_sq
    }
}