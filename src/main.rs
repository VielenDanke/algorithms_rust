pub mod linked_list;
pub mod graph;
pub mod strings;
pub mod sort;
pub mod dynamic_programming;
pub mod array;
pub mod backtracking;
pub mod stack;

fn main() {
    let result = backtracking::hard::word_break_2_140::Solution::word_break("catsanddog".to_string(), vec!["cat".to_string(), "cats".to_string(), "and".to_string(), "sand".to_string(), "dog".to_string()]);
    println!("{:?}", result);
}
