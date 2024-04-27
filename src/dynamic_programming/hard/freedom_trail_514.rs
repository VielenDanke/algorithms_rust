use std::collections::{HashMap, HashSet};

pub struct Solution {}

impl Solution {
    fn count_steps(current: usize, next: usize, ring_length: usize) -> i32 {
        let steps_between = current.abs_diff(next);
        let steps_around = ring_length - steps_between;
        steps_between.min(steps_around) as i32
    }

    fn try_lock(cache: &mut Vec<Vec<i32>>, ring_index: usize, key_index: usize, ring: &[u8], key: &[u8]) -> i32 {
        if cache[ring_index][key_index] < 1 << 30 {
            return cache[ring_index][key_index];
        }
        if key_index == key.len() {
            return 0;
        }
        let mut min_steps = 1 << 30;
        for i in 0..ring.len() {
            if ring[i] == key[key_index] {
                let total_steps = Solution::count_steps(ring_index, i, ring.len()) +
                    1 +
                    Solution::try_lock(cache, i, key_index + 1, ring, key);
                min_steps = min_steps.min(total_steps);
                cache[ring_index][key_index] = min_steps;
            }
        }
        min_steps
    }

    pub fn find_rotate_steps_top_down(ring: String, key: String) -> i32 {
        let mut cache = vec![vec![1 << 30; key.len() + 1]; ring.len() + 1];
        Solution::try_lock(&mut cache, 0, 0, ring.as_bytes(), key.as_bytes())
    }

    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let nr = ring.len();
        let nk = key.len();

        let cixs = ring.chars().enumerate().fold(
            HashMap::<char, HashSet<usize>>::new(),
            |mut a, (i, x)| {
                a.entry(x).or_insert_with(HashSet::new).insert(i);
                a
            },
        );

        let mut layer: Vec<(usize, i32)> = vec![(0, 0)];

        for c in key.chars() {
            let mut next_layer: Vec<(usize, i32)> = Vec::new();
            if let Some(ixs) = cixs.get(&c) {
                for &ix in ixs.iter() {
                    let mut minlength = i32::MAX;
                    for &(pix, ppath) in layer.iter() {
                        let diff = if ix < pix { pix - ix } else { ix - pix };
                        let dist = diff.min(nr - diff);
                        minlength = minlength.min(ppath + dist as i32);
                    }
                    next_layer.push((ix, minlength));
                }
            }
            layer = next_layer;
        }

        layer.iter().map(|x| x.1).min().unwrap() + nk as i32
    }
}
