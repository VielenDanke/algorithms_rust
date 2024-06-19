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
    let result = sort::medium::minimum_number_of_days_to_make_m_bouquets_1482::Solution::min_days(vec![1, 10, 3, 10, 2], 3, 1);
    println!("{}", result);
}
