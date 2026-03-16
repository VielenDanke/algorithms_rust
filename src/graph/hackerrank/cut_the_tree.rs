use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::collections::HashMap;

/*
 * Complete the 'cutTheTree' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY data
 *  2. 2D_INTEGER_ARRAY edges
 */

fn cut_the_tree(data: &[i32], edges: &[Vec<i32>]) -> i32 {
    let mut graph = HashMap::new();

    let total_sum = data.iter().sum::<i32>();

    for edge in edges.iter() {
        graph.entry(edge[0]).or_insert(Vec::new()).push(edge[1]);
        graph.entry(edge[1]).or_insert(Vec::new()).push(edge[0]);
    }

    fn dfs(n: i32, p: i32, data: &[i32], graph: &HashMap<i32, Vec<i32>>, min: &mut i32, total_sum: i32) -> i32 {
        let mut current_sum = data[(n - 1) as usize];

        if let Some(next_nodes) = graph.get(&n) {
            for &next_node in next_nodes.iter() {
                current_sum += dfs(next_node, n, data, graph, min, total_sum);
            }
        }

        if p != -1 {
            let mut diff = i32::abs(total_sum - 2 * current_sum);
            *min = *min.min(&mut diff);
        }

        current_sum
    }

    let mut min = i32::MAX;

    dfs(0, -1, data, &graph, &mut min, total_sum);

    min
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let data: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let mut edges: Vec<Vec<i32>> = Vec::with_capacity((n - 1) as usize);

    for i in 0..(n - 1) as usize {
        edges.push(Vec::with_capacity(2_usize));

        edges[i] = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = cut_the_tree(&data, &edges);

    writeln!(&mut fptr, "{}", result).ok();
}
