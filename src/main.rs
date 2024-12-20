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
    let i = array::medium::task_769::Solution::max_chunks_to_sorted(
        vec![2,0,1,3],
    );
    println!("{}", i);
}
