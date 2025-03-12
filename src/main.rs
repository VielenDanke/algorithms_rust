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
    let i = array::medium::task_3306::Solution::count_of_substrings("iqeaouqi".to_string(), 2);
    println!("{:?}", i);
}
