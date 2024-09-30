use std::collections::BTreeSet;

struct MyCalendar {
    set: BTreeSet<(i32, i32)>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {

    fn new() -> Self {
        MyCalendar {
            set: BTreeSet::new()
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        let incoming_period = (start, end);

        if self.set.contains(&incoming_period) {
            return false;
        }

        self.set.insert(incoming_period);

        let mut last: Option<(i32, i32)> = None;

        for period in self.set.iter() {
            if last.is_some() && last.unwrap().1 > period.0 {
                self.set.remove(&incoming_period);
                return false;
            }
            last = Some(period.clone());
        }
        true
    }
}