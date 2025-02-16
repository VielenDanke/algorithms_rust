use std::collections::HashMap;

struct NumberContainersTle {
    numbers: Vec<Vec<Option<(i32, usize)>>>,
}

impl NumberContainersTle {
    fn new() -> Self {
        NumberContainersTle {
            numbers: vec![Vec::new(); 100],
        }
    }

    fn change(&mut self, index: i32, number: i32) {
        let u_index = (index % 100) as usize;

        let bucket = self.numbers.get_mut(u_index).unwrap();

        let bucket_len = bucket.len();

        for i in 0..bucket_len {
            let current_bucket_pair = bucket.get(i).unwrap();
            if current_bucket_pair.is_some() && current_bucket_pair.unwrap().1 == index as usize {
                bucket[i] = Some((number, index as usize));
                return;
            }
        }
        bucket.push(Some((number, index as usize)));
    }

    fn find(&self, number: i32) -> i32 {
        let mut min_index = None;
        for bucket in self.numbers.iter() {
            for bucket_opt in bucket.iter() {
                if let Some((bucket_number, index)) = bucket_opt {
                    if number == *bucket_number {
                        if min_index.is_none() {
                            min_index = Some(*index as i32);
                        } else {
                            min_index = Some(min_index.unwrap().min(*index as i32));
                        }
                    }
                }
            }
        }
        min_index.unwrap_or(-1)
    }
}

// ---------------------------------------------------------------------------------------------------------

use std::collections::BTreeSet;

struct NumberContainers {
    hash: HashMap<i32, i32>,
    numbers: HashMap<i32, BTreeSet<i32>>,
}

impl NumberContainers {
    fn new() -> Self {
        NumberContainers {
            hash: HashMap::new(),
            numbers: HashMap::new(),
        }
    }

    fn change(&mut self, index: i32, number: i32) {
        if self.hash.contains_key(&index) {
            if self.numbers.contains_key(&self.hash[&index]) {
                self.numbers
                    .get_mut(&self.hash[&index])
                    .unwrap()
                    .remove(&index);
            }
        }
        *self.hash.entry(index).or_insert(number) = number;
        self.numbers
            .entry(number)
            .or_insert(BTreeSet::new())
            .insert(index);
    }

    fn find(&self, number: i32) -> i32 {
        if self.numbers.contains_key(&number) {
            if let Some(idx) = self.numbers[&number].first() {
                return *idx;
            }
        }
        -1
    }
}
