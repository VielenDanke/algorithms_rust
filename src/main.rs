pub mod linked_list;
pub mod graph;
pub mod strings;
pub mod sort;
pub mod dynamic_programming;
pub mod array;
pub mod backtracking;
pub mod stack;

fn main() {
    let result = strings::medium::append_characters_to_string_to_make_subsequence_2486::Solution::append_characters("coaching".to_string(), "coding".to_string());
    println!("{}", result);
}
