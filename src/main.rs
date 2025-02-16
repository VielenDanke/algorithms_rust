pub mod linked_list;
pub mod graph;
pub mod strings;
pub mod sort;
pub mod dynamic_programming;
pub mod array;
pub mod backtracking;
pub mod stack;
pub mod math;
pub mod structures;

fn main() {
    let i = array::medium::task_3066::Solution::min_operations(vec![1, 1, 2, 4, 9], 20);
    println!("{:?}", i);
}
