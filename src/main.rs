pub mod linked_list;
pub mod graph;
pub mod strings;
pub mod sort;
pub mod dynamic_programming;
pub mod array;
pub mod backtracking;
pub mod stack;
pub mod math;

fn main() {
    let i = array::medium::task_1760::Solution::minimum_size(
        vec![9],
        2,
    );
    println!("{}", i);
}
