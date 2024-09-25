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
    strings::medium::task_3043::Solution::longest_common_prefix_trie(vec![1, 10, 100], vec![1000]);
}
