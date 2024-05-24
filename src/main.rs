pub mod linked_list;
pub mod graph;
pub mod strings;
pub mod sort;
pub mod dynamic_programming;
pub mod array;
pub mod backtracking;
pub mod stack;

fn main() {
    let mut result = backtracking::hard::maximum_score_words_formed_by_letters_1255::Solution::max_score_words(
        vec!["dog".to_string(), "cat".to_string(), "dad".to_string(), "good".to_string()],
        vec!['a', 'a', 'c', 'd', 'd', 'd', 'g', 'o', 'o'],
        vec![1, 0, 9, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    );
    println!("{}", result);
}
