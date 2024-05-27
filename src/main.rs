pub mod linked_list;
pub mod graph;
pub mod strings;
pub mod sort;
pub mod dynamic_programming;
pub mod array;
pub mod backtracking;
pub mod stack;

fn main() {
    let result = array::easy::special_array_with_x_elements_greater_than_or_equal_x_1608::Solution::special_array(vec![0,4,3,0,4]);
    println!("{}", result);
}
