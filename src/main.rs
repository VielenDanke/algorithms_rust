pub mod linked_list;
pub mod graph;
pub mod strings;
pub mod sort;
pub mod dynamic_programming;
pub mod array;
pub mod backtracking;
pub mod stack;

fn main() {
    let i = strings::medium::compare_version_numbers_165::Solution::compare_version("1.01".to_string(), "1.001".to_string());
    println!("{i}");
}
