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
    let result = strings::medium::minimum_cost_to_convert_string_2976::Solution::minimum_cost(
        "abcd".to_string(),
        "acbe".to_string(),
        vec!['a','b','c','c','e','d'],
        vec!['b','c','b','e','b','e'],
        vec![2,5,5,1,2,20],
    );

    print!("{}", result);
}
