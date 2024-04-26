pub mod linked_list;
pub mod graph;
pub mod strings;
pub mod sort;
pub mod dynamic_programming;
pub mod array;
pub mod backtracking;
pub mod stack;

fn main() {
    println!("{:?}", dynamic_programming::hard::minimum_falling_path_sum_2_1289::Solution::min_falling_path_sum(vec![vec![7]]));
}
