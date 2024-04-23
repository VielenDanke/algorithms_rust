pub mod linked_list;
pub mod graph;
pub mod strings;
pub mod sort;
pub mod dynamic_programming;
pub mod array;
pub mod backtracking;
pub mod stack;

fn main() {
    println!("{:?}", graph::medium::minimum_height_trees_310::Solution::find_min_height_trees_bfs(1, vec![]));
}
