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
    let result = array::hard::task_440::Solution::find_kth_number(804289384, 42641503);
    println!("{:?}", result);
}
