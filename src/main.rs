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
    let diverse_string = backtracking::medium::task_1405::Solution::longest_diverse_string(1, 1, 7);
    println!("{diverse_string}");
}
