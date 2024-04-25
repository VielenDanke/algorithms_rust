pub mod linked_list;
pub mod graph;
pub mod strings;
pub mod sort;
pub mod dynamic_programming;
pub mod array;
pub mod backtracking;
pub mod stack;

fn main() {
    println!("{:?}", dynamic_programming::medium::longest_ideal_subsequence_2370::Solution::longest_ideal_string(String::from("abcd"), 3));
}
