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
    let result = array::medium::task_386::Solution::lexical_order(13);
    println!("{:?}", result);
}
