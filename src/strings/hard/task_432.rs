use std::collections::HashMap;

struct AllOne {
    counter: HashMap<String, i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AllOne {

    fn new() -> Self {
        AllOne {
            counter: HashMap::new(),
        }
    }

    fn inc(&mut self, key: String) {
        *self.counter.entry(key).or_default() += 1;
    }

    fn dec(&mut self, key: String) {
        let entry = self.counter.entry(key.clone()).or_default();
        *entry -= 1;
        if *entry == 0 {
            self.counter.remove(&key);
        }
    }

    fn get_max_key(&self) -> String {
        let mut max = None;
        let mut result = String::new();
        for (k, &v) in self.counter.iter() {
            if max.is_none() || max.unwrap() < v {
                max = Some(v);
                result = k.clone();
            }
        }
        result
    }

    fn get_min_key(&self) -> String {
        let mut min = None;
        let mut result = String::new();
        for (k, &v) in self.counter.iter() {
            if min.is_none() || min.unwrap() > v {
                min = Some(v);
                result = k.clone();
            }
        }
        result
    }
}

/**
 * Your AllOne object will be instantiated and called as such:
 * let obj = AllOne::new();
 * obj.inc(key);
 * obj.dec(key);
 * let ret_3: String = obj.get_max_key();
 * let ret_4: String = obj.get_min_key();
 */