pub mod linked_list;
pub mod graph;
pub mod strings;
pub mod sort;
pub mod dynamic_programming;
pub mod array;
pub mod backtracking;
pub mod stack;

fn main() {
    let result = graph::medium::path_with_minimum_effort_1631::Solution::minimum_effort_path(vec![vec![1,2,2],vec![3,8,2],vec![5,3,5]]);
    println!("{}", result);
}
