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
    let result = stack::medium::reverse_substrings_between_each_pair_of_parentheses_1190::Solution::reverse_parentheses("(ed(et(oc))el)".to_string());

    println!("{result}");
}
