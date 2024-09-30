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
        let counter_opt = self.exists.get(&key);
        if counter_opt.is_some() {
            let mut counter = *counter_opt.unwrap();
            let current_counter = self.counter.entry(counter).or_default();
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
            self.counter.entry(counter).or_default().insert(key.clone());
        }
    }

    fn dec(&mut self, key: String) {
        let counter_opt = self.exists.get(&key);
        if counter_opt.is_some() {
            let mut counter = *counter_opt.unwrap();
            let current_counter = self.counter.entry(counter).or_default();
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