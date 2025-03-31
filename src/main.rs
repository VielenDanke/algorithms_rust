pub mod array;
pub mod backtracking;
pub mod dynamic_programming;
pub mod graph;
pub mod linked_list;
pub mod math;
pub mod sort;
pub mod stack;
pub mod strings;
pub mod structures;

fn main() {
    let i = sort::medium::task_2780::Solution::minimum_index(vec![1, 2, 2, 2]);
    println!("{:?}", i);
}
