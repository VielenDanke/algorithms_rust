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
    let i = array::easy::task_1863::Solution::subset_xor_sum(vec![5, 1, 6]);
    println!("{:?}", i);
}
