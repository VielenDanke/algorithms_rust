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
    let result = array::medium::count_number_of_teams_1395::Solution::num_teams(vec![2,5,3,4,1]);

    print!("{}", result);
}
