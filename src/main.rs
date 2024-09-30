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
    let mut stack = array::medium::task_1381::CustomStack::new(3);
    stack.push(1);
    stack.push(2);
    stack.pop();
    stack.push(2);
    stack.push(3);
    stack.push(4);
    println!("{:?}", stack);
    stack.increment(2, 100);
    stack.increment(5, 100);
    println!("{:?}", stack);
    stack.pop();
    stack.pop();
    stack.pop();
    stack.pop();
}
