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
    let result = array::medium::longest_continuous_subarray_with_absolute_diff_less_than_or_equal_to_limit_1438::Solution::longest_subarray(vec![1,5,6,7,8,10,6,5,6], 4);
    println!("{}", result);
}
