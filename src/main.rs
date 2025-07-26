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
    println!("{:?}", strings::medium::task_1233::Solution::remove_subfolders(vec!["/a".to_string(),"/a/b".to_string(),"/c/d".to_string(),"/c/d/e".to_string(),"/c/f".to_string()]));
}
