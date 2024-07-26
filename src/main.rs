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
    let result = graph::medium::find_the_city_with_the_smallest_number_of_neighbors_at_threshold_distance_1334::Solution::find_the_city(
        5, vec![vec![0,1,2],vec![0,4,8],vec![1,2,3],vec![1,4,2],vec![2,3,1],vec![3,4,1]], 2,
    );

    println!("{}", result);
}
