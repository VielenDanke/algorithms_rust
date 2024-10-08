use std::collections::VecDeque;

struct MyCircularDeque {
    dequeue: VecDeque<i32>,
    capacity: i32,
    initial_size: i32,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularDeque {

    fn new(k: i32) -> Self {
        MyCircularDeque {
            dequeue: VecDeque::new(),
            capacity: k,
            initial_size: k,
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.dequeue.push_front(value);
        self.capacity -= 1;
        true
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.dequeue.push_back(value);
        self.capacity -= 1;
        true
    }

    fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.dequeue.pop_front();
        self.capacity += 1;
        true
    }

    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.dequeue.pop_back();
        self.capacity += 1;
        true
    }

    fn get_front(&self) -> i32 {
        if self.is_empty() {
            return -1
        }
        *self.dequeue.front().unwrap()
    }

    fn get_rear(&self) -> i32 {
        if self.is_empty() {
            return -1
        }
        *self.dequeue.back().unwrap()
    }

    fn is_empty(&self) -> bool {
        self.capacity == self.initial_size
    }

    fn is_full(&self) -> bool {
        self.capacity == 0
    }
}