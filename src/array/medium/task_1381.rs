struct CustomStack {
    stack: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CustomStack {
    fn new(max_size: i32) -> Self {
        CustomStack {
            stack: Vec::with_capacity(max_size as usize),
        }
    }

    fn push(&mut self, x: i32) {
        if self.stack.len() + 1 <= self.stack.capacity() {
            self.stack.push(x);
        }
    }

    fn pop(&mut self) -> i32 {
        self.stack.pop().unwrap_or(-1)
    }

    fn increment(&mut self, k: i32, val: i32) {
        for num in self.stack.iter_mut().take(k as usize) {
            *num += val;
        }
    }
}