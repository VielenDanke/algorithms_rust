pub mod linked_list;
pub mod graph;
pub mod strings;
pub mod sort;
pub mod dynamic_programming;
pub mod array;
pub mod backtracking;
pub mod stack;

fn main() {
    let result = backtracking::hard::check_record_552::Solution::check_record(10101);
    println!("{}", result);
}
