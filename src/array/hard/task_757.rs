use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn intersection_size_two(mut intervals: Vec<Vec<i32>>) -> i32 {
        // Sort intervals based on:
        // 1. End point (Ascending) - to process intervals that finish early first.
        // 2. Start point (Descending) - if ends are equal, process the shortest one first
        //    to maximize the constraints early.
        intervals.sort_unstable_by(|a, b| {
            if a[1] == b[1] {
                b[0].cmp(&a[0])
            } else {
                a[1].cmp(&b[1])
            }
        });

        let mut result = 0;

        // p1 is the second-largest number added to the set
        // p2 is the largest number added to the set
        // Initialize with -1 (assuming inputs are positive)
        let mut p1 = -1;
        let mut p2 = -1;

        for interval in intervals {
            let start = interval[0];
            let end = interval[1];

            // Case 1: No overlap
            // The current start is greater than our largest existing point.
            // We need to add 2 points. Best strategy: pick the largest possible (end, end-1)
            // to maximize overlap with future intervals.
            if start > p2 {
                result += 2;
                p1 = end - 1;
                p2 = end;
            }
            // Case 2: Partial overlap (1 point)
            // The current start is > p1 but <= p2.
            // p2 is inside this interval, but p1 is not. We need 1 more point.
            // We pick 'end' to maximize future overlap.
            else if start > p1 {
                result += 1;
                p1 = p2; // The old max becomes the second max
                p2 = end; // The current end becomes the new max
            }
            // Case 3: Full overlap (2 points)
            // start <= p1. Since p1 < p2, both p1 and p2 are inside this interval.
            // We don't need to add anything.
        }

        result
    }

    pub fn intersection_size_two_brute_force(intervals: Vec<Vec<i32>>) -> i32 {
        // count each number and what buckets cover
        // sort descending by number of covered buckets and pop first numbers

        let n = intervals.len();

        let mut m = HashMap::new();

        for i in 0..n {
            for num in intervals[i][0]..=intervals[i][1] {
                m.entry(num).or_insert(Vec::new()).push(i);
            }
        }
        let mut result = 0;

        let mut bucket_counter = HashMap::new();

        for i in 0..n {
            bucket_counter.entry(i).or_insert(2);
        }

        while !bucket_counter.is_empty() {
            let mut max_num = -1 << 30;
            let mut max_len = 0;
            for (&k, v) in m.iter() {
                if max_len < v.len() {
                    max_len = v.len();
                    max_num = k;
                }
            }
            if max_num == -1 << 30 {
                break;
            }
            let max_vec = m.remove(&max_num).unwrap();

            let mut is_hit = false;

            for v in max_vec {
                let current = *bucket_counter.get(&v).unwrap_or(&-1);

                if current == -1 {
                    continue;
                }
                is_hit = true;

                if current <= 1 {
                    bucket_counter.remove(&v);
                } else {
                    bucket_counter.insert(v, current - 1);
                }
            }
            if is_hit {
                result += 1;
            }
        }

        result
    }
}
