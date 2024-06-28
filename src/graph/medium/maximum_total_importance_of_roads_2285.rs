pub struct Solution;

impl Solution {
    pub fn maximum_importance_shorter(n: i32, mut roads: Vec<Vec<i32>>) -> i64 {
        let mut graph = roads.iter().fold(vec![0i64; n as usize], |mut acc, road| {
            acc[road[0] as usize] += 1;
            acc[road[1] as usize] += 1;
            acc
        });

        graph.sort_unstable();

        let mut total = 0;

        for i in 1..=n {
            total += graph[i as usize - 1] * i as i64
        }

        total
    }

    pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;
        let mut connections = vec![0i64; n];

        for road in roads.iter() {
            connections[road[0] as usize] += 1;
            connections[road[1] as usize] += 1;
        }

        let mut l_roads = (0..n).into_iter().collect::<Vec<usize>>();

        l_roads.sort_unstable_by(|left, right| connections[*right].cmp(&connections[*left]));

        let mut weight = n as i64;

        for road in l_roads.iter() {
            connections[*road] = weight;
            weight -= 1;
        }

        let mut result = 0i64;

        for road in roads.iter() {
            result += connections[road[0] as usize] + connections[road[1] as usize];
        }

        result
    }
}
