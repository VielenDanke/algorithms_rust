struct MyCalendarTwo {
    b: Vec<(i32, i32)>,
    db: Vec<(i32, i32)>,
}

impl MyCalendarTwo {
    fn new() -> Self {
        Self { b: vec![], db: vec![] }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        for &(s, e) in self.db.iter() {
            if start < e && end > s { return false; }
        }
        for &(s, e) in self.b.iter() {
            if start < e && end > s {
                self.db.push((start.max(s), end.min(e)));
            }
        }
        self.b.push((start, end));
        true
    }
}