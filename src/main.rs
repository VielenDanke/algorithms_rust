pub mod linked_list;
pub mod graph;
pub mod strings;
pub mod sort;
pub mod dynamic_programming;
pub mod array;
pub mod backtracking;
pub mod stack;

fn main() {
    let result = backtracking::medium::find_the_safest_path_in_a_grid_2812::Solution::maximum_safeness_factor(vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 1]]);
    println!("{}", result);
}
