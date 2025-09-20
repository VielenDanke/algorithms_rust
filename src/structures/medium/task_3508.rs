use std::collections::{HashMap, VecDeque, HashSet};

struct Router {
    memory_limit: usize,
    q: VecDeque<(i32, i32, i32)>,  // (source, destination, timestamp)
    router: HashSet<(i32, i32, i32)>,
    dest: HashMap<i32, Vec<i32>>,  // destination -> sorted timestamps
}

impl Router {
    /// Initialize router with memory limit
    /// TC: O(1), SC: O(1)
    fn new(memory_limit: i32) -> Self {
        Router {
            memory_limit: memory_limit as usize,
            q: VecDeque::new(),
            router: HashSet::new(),
            dest: HashMap::new(),
        }
    }

    /// Add packet with duplicate check and memory management
    /// TC: O(log k), SC: O(1) amortized
    fn add_packet(&mut self, source: i32, destination: i32, timestamp: i32) -> bool {
        let packet = (source, destination, timestamp);

        // Check for duplicate
        if self.router.contains(&packet) {
            return false;
        }

        // Handle memory limit
        if self.q.len() >= self.memory_limit {
            if let Some((old_source, old_dest, old_timestamp)) = self.q.pop_front() {
                let old_packet = (old_source, old_dest, old_timestamp);
                self.router.remove(&old_packet);

                // Remove from destination timestamps
                if let Some(timestamps) = self.dest.get_mut(&old_dest) {
                    if let Some(pos) = timestamps.iter().position(|&x| x == old_timestamp) {
                        timestamps.remove(pos);
                    }
                    if timestamps.is_empty() {
                        self.dest.remove(&old_dest);
                    }
                }
            }
        }

        // Add new packet
        self.q.push_back(packet);
        self.router.insert(packet);

        // Insert timestamp in sorted order
        let timestamps = self.dest.entry(destination).or_insert_with(Vec::new);
        let insert_pos = timestamps.binary_search(&timestamp).unwrap_or_else(|x| x);
        timestamps.insert(insert_pos, timestamp);

        true
    }

    /// Forward oldest packet
    /// TC: O(log k), SC: O(1)
    fn forward_packet(&mut self) -> Vec<i32> {
        if let Some((source, destination, timestamp)) = self.q.pop_front() {
            let packet = (source, destination, timestamp);
            self.router.remove(&packet);

            // Remove from destination timestamps
            if let Some(timestamps) = self.dest.get_mut(&destination) {
                if let Some(pos) = timestamps.iter().position(|&x| x == timestamp) {
                    timestamps.remove(pos);
                }
                if timestamps.is_empty() {
                    self.dest.remove(&destination);
                }
            }

            vec![source, destination, timestamp]
        } else {
            vec![]
        }
    }

    /// Count packets in time range using binary search
    /// TC: O(log k), SC: O(1)
    fn get_count(&self, destination: i32, start_time: i32, end_time: i32) -> i32 {
        if let Some(timestamps) = self.dest.get(&destination) {
            let left_idx = timestamps.binary_search(&start_time).unwrap_or_else(|x| x);
            let right_idx = timestamps.binary_search(&(end_time + 1)).unwrap_or_else(|x| x);
            (right_idx - left_idx) as i32
        } else {
            0
        }
    }
}