pub mod array;
pub mod backtracking;
pub mod dynamic_programming;
pub mod graph;
pub mod linked_list;
pub mod math;
pub mod sort;
pub mod stack;
pub mod strings;
pub mod structures;

fn main() {
    let i = array::medium::task_3169::Solution::count_days_line_sort_merge(
        57,
        vec![
            vec![3, 49],
            vec![23, 44],
            vec![21, 56],
            vec![26, 55],
            vec![23, 52],
            vec![2, 9],
            vec![1, 48],
            vec![3, 31],
        ],
    );
    println!("{:?}", i);
}
