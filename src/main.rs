pub mod linked_list;
pub mod graph;
pub mod strings;
pub mod sort;
pub mod dynamic_programming;
pub mod array;
pub mod backtracking;
pub mod stack;

fn main() {
    println!("{}", strings::medium::open_the_lock_752::Solution::open_lock(vec!["0201".to_string(), "0101".to_string(), "0102".to_string(), "1212".to_string(), "2002".to_string()], "0202".to_string()));
}
