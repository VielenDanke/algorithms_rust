use std::collections::{HashMap, HashSet, VecDeque};

struct Router {
    memory_limit: usize,
    queue: VecDeque<(i32, i32, i32)>, // (source, destination, timestamp)
    seen: HashSet<String>,            // duplicate detection
    dest_map: HashMap<i32, Vec<i32>>, // destination -> timestamps
}

impl Router {
    fn make_key(source: i32, dest: i32, timestamp: i32) -> String {
        format!("{},{},{}", source, dest, timestamp)
    }

    fn new(memory_limit: i32) -> Self {
        Router {
            memory_limit: memory_limit as usize,
            queue: VecDeque::new(),
            seen: HashSet::new(),
            dest_map: HashMap::new(),
        }
    }

    fn add_packet(&mut self, source: i32, destination: i32, timestamp: i32) -> bool {
        let key = Self::make_key(source, destination, timestamp);
        if self.seen.contains(&key) {
            return false; // duplicate
        }

        if self.queue.len() == self.memory_limit {
            self.forward_packet(); // evict oldest
        }

        self.queue.push_back((source, destination, timestamp));
        self.seen.insert(key);
        self.dest_map.entry(destination).or_insert(vec![]).push(timestamp);
        true
    }

    fn forward_packet(&mut self) -> Vec<i32> {
        if let Some((s, d, t)) = self.queue.pop_front() {
            let key = Self::make_key(s, d, t);
            self.seen.remove(&key);

            if let Some(times) = self.dest_map.get_mut(&d) {
                if !times.is_empty() {
                    times.remove(0); // O(n), usually small
                }
                if times.is_empty() {
                    self.dest_map.remove(&d);
                }
            }

            vec![s, d, t]
        } else {
            vec![]
        }
    }

    fn get_count(&self, destination: i32, start_time: i32, end_time: i32) -> i32 {
        if let Some(times) = self.dest_map.get(&destination) {
            times.iter().filter(|&&t| t >= start_time && t <= end_time).count() as i32
        } else {
            0
        }
    }
}