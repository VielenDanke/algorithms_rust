struct CustomStack {
    stack: Vec<i32>,
    size: i32,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CustomStack {
    fn new(max_size: i32) -> Self {
        CustomStack {
            stack: Vec::with_capacity(max_size as usize),
            size: max_size,
        }
    }

    fn push(&mut self, x: i32) {
        if self.size == 0 {
            return;
        }
        self.stack.push(x);
        self.size -= 1;
    }

    fn pop(&mut self) -> i32 {
        if self.stack.is_empty() {
            -1
        } else {
            self.size += 1;
            self.stack.pop().unwrap()
        }
    }

    fn increment(&mut self, k: i32, val: i32) {
        for num in self.stack.iter_mut().take(k as usize) {
            *num += val;
        }
    }
}