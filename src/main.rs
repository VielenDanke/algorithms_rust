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
    let i = array::easy::task_1752::Solution::check(
        vec![1, 2, 3],
    );
    println!("{}", i);
}
