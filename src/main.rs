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
    let i = array::medium::task_2401::Solution::longest_nice_subarray(vec![1,3,8,48,10]);
    println!("{:?}", i);
}
