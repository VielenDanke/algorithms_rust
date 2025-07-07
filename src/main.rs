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
    println!("{}", array::medium::task_1353::Solution::max_events(vec![vec![1,4],vec![4,4],vec![2,2],vec![3,4],vec![1,1]]));
}
