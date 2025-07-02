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
    println!("{}", strings::medium::task_3330::Solution::possible_string_count("ere".to_string()));
}
