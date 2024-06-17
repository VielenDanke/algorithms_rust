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
    let result = array::medium::hand_of_straights_846::Solution::is_n_straight_hand(vec![1,2,3,6,2,3,4,7,8], 3);
    println!("{}", result);
}
