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
    let result = graph::medium::all_ancestors_of_node_in_directec_acyclic_graph_2192::Solution::get_ancestors(
        6, vec![vec![0, 3], vec![5, 0], vec![2, 3], vec![4, 3], vec![5, 3], vec![1, 3], vec![2, 5], vec![0, 1], vec![4, 5], vec![4, 2], vec![4, 0], vec![2, 1], vec![5, 1]],
    );
    println!("{:?}", result);
}
