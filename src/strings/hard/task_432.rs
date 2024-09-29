use std::collections::{BTreeMap, BTreeSet, HashMap};

struct AllOne {
    counter: BTreeMap<i32, BTreeSet<String>>,
    exists: HashMap<String, i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AllOne {
    fn new() -> Self {
        AllOne {
            counter: BTreeMap::new(),
            exists: HashMap::new(),
        }
    }

    fn inc(&mut self, key: String) {
        if self.exists.contains_key(&key) {
            let mut counter = *self.exists.get(&key).unwrap();
            let current_counter = self.counter.entry(counter).or_insert(BTreeSet::new());
            if current_counter.contains(&key) {
                current_counter.remove(&key);
                if current_counter.len() == 0 {
                    self.counter.remove(&counter);
                }
                counter += 1;
                self.counter.entry(counter).or_default().insert(key.clone());
                self.exists.insert(key.clone(), counter);
            }
        } else {
            let counter = 1;
            self.exists.insert(key.clone(), counter);
            self.counter.entry(counter).or_insert(BTreeSet::new()).insert(key.clone());
        }
    }

    fn dec(&mut self, key: String) {
        if self.exists.contains_key(&key) {
            let mut counter = *self.exists.get(&key).unwrap();
            let current_counter = self.counter.entry(counter).or_insert(BTreeSet::new());
            current_counter.remove(&key);
            if current_counter.len() == 0 {
                self.counter.remove(&counter);
            }
            if counter == 1 {
                self.exists.remove(&key);
            } else {
                counter -= 1;
                self.exists.insert(key.clone(), counter);
                self.counter.entry(counter).or_default().insert(key.clone());
            }
        }
    }

    fn get_max_key(&mut self) -> String {
        if self.counter.len() == 0 {
            return String::new()
        }
        self.counter.last_entry().unwrap().get().first().unwrap().clone()
    }

    fn get_min_key(&mut self) -> String {
        if self.counter.len() == 0 {
            return String::new()
        }
        self.counter.first_entry().unwrap().get().first().unwrap().clone()
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